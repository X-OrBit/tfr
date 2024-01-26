mod files_environment;

use crate::files_environment::{FilesEnvironment, FilesEnvironmentConfig};
use tfr::TemplateFileRenamer;
use std::io;
use std::io::Read;

mod integration_tests {
    use super::*;

    fn read_environment_config(config_path: &str) -> Result<FilesEnvironmentConfig, io::Error> {
        let mut file = std::fs::File::open(config_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        Ok(serde_json::from_str::<FilesEnvironmentConfig>(&data)
            .expect("JSON was not well-formatted"))
    }

    fn test_with_json_config(config_path: &str) {
        let config_path = format!("tests/tests/{}", config_path);
        let environment_config = read_environment_config(&config_path).unwrap();
        let files_environment = FilesEnvironment::new(&environment_config).unwrap();

        let tfr = TemplateFileRenamer::new(environment_config.action_when_exists.clone().into());

        let result = tfr.rename(
            &files_environment.get_full_path(&environment_config.input_template),
            &files_environment.get_full_path(&environment_config.output_template),
        );
        let is_correct_status = match result {
            Err(_) => environment_config.raise_error,
            Ok(_) => !environment_config.raise_error,
        };

        assert!(is_correct_status && files_environment.is_after())
    }

    #[test]
    fn simple_test() {
        test_with_json_config("simple.json");
    }

    #[test]
    fn rename_test() {
        test_with_json_config("png.json");
    }

    #[test]
    fn multiple_asterisks_test() {
        test_with_json_config("multiple_asterisks.json");
    }

    #[test]
    fn flag_before_last_part_test() {
        test_with_json_config("flag_before_last_part.json");
    }

    #[test]
    fn escaped_asterisk_test() {
        test_with_json_config("escaped_asterisk.json");
    }

    #[test]
    fn directory_test() {
        test_with_json_config("directory.json");
    }

    #[test]
    fn output_template_with_hash_test() {
        test_with_json_config("output_template_with_hash.json");
    }

    #[test]
    fn nothing_to_rename_test() {
        test_with_json_config("nothing_to_rename/simple.json");
        test_with_json_config("nothing_to_rename/png.json");
    }

    #[test]
    fn terminate_test() {
        test_with_json_config("terminate.json");
    }

    #[test]
    fn terminate_existing_dir_test() {
        test_with_json_config("terminate2.json");
    }

    #[test]
    fn templates_error_test() {
        test_with_json_config("templates_error/asterisk_before_last_part.json");
        test_with_json_config("templates_error/double_asterisk.json");
        test_with_json_config("templates_error/captures_not_covered_by_flags.json");
    }

    #[test]
    fn skip_when_exists_test() {
        test_with_json_config("skip_when_exists.json");
    }

    #[test]
    fn overwrite_when_exists_test() {
        test_with_json_config("overwrite_when_exists.json");
    }
}
