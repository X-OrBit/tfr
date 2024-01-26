use itertools::Itertools;
use regex::Regex;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::mmv::file_utils::{resolve_path_pattern, Template};
use crate::mmv::{ActionWhenRenamedFilePathExists, TfrError};

pub fn is_rename_template_correct(input_file_template: &str, output_file_template: &str) -> bool {
    let asterisk_count =
        input_file_template.matches('*').count() - input_file_template.matches("\\*").count();
    let placement_regex = Regex::new(r#"#(\d+)"#).unwrap();
    let correct_unique_flag_count: usize = placement_regex
        .captures_iter(output_file_template)
        .filter_map(|capture| {
            let index = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
            if 1 <= index && index <= asterisk_count {
                Some(index)
            } else {
                None
            }
        })
        .unique()
        .count();

    correct_unique_flag_count >= asterisk_count
}

pub fn apply_template(
    input_file_template: &str,
    output_file_template: &str,
    rename_mod: &ActionWhenRenamedFilePathExists,
) -> Result<Vec<(String, String)>, TfrError> {
    if !is_rename_template_correct(input_file_template, output_file_template) {
        return Err(TfrError::IncorrectOutputTemplate(
            "Output template flags does not cover input template asterisks",
        ));
    }

    let input_dir =
        Path::new(input_file_template)
            .parent()
            .ok_or(TfrError::IncorrectInputTemplate(
                "Empty input template does not allowed",
            ))?;
    let input_dir = fs::read_dir(input_dir).map_err(|_| {
        TfrError::IncorrectInputTemplate("Input template parent directory not found")
    })?;

    let input_file_template = Template::new(input_file_template)?;

    let extract_file_path = |path: DirEntry| {
        if path.file_type().unwrap().is_file() {
            return Some(path.path());
        }
        None
    };
    let file_candidates = input_dir
        .filter_map(|entry: io::Result<DirEntry>| entry.ok().and_then(extract_file_path))
        .collect::<Vec<PathBuf>>();

    let mut existing_path: Option<String> = None;

    let mut apply_template_to_filepath =
        |input_path: &str, captures: Vec<&str>| -> Option<(String, String)> {
            let new_filepath = resolve_path_pattern(output_file_template, captures);
            if !Path::new(&new_filepath).exists() {
                return Some((input_path.to_string(), new_filepath));
            }
            if Path::new(&new_filepath).is_dir() {
                existing_path = Some(new_filepath.to_string());
                return None;
            }
            match rename_mod {
                ActionWhenRenamedFilePathExists::Terminate => {
                    existing_path = Some(new_filepath.to_string());
                    None
                }
                ActionWhenRenamedFilePathExists::Skip => None,
                ActionWhenRenamedFilePathExists::Overwrite => {
                    Some((input_path.to_string(), new_filepath))
                }
            }
        };

    let applied_new_filepaths: Vec<(String, String)> = file_candidates
        .iter()
        .filter_map(|input_path: &PathBuf| {
            let input_path = input_path.to_str().unwrap().to_string();
            input_file_template
                .captures(&input_path)
                .and_then(|captures| apply_template_to_filepath(&input_path, captures))
        })
        .collect();

    match existing_path {
        None => Ok(applied_new_filepaths),
        Some(existing_path) => Err(TfrError::ExistingPath(
            existing_path.clone(),
            Path::new(&existing_path).is_file(),
        )),
    }
}
