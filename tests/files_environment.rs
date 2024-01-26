use tfr::ActionWhenRenamedFilePathExists;
use serde::Deserialize;
use std::collections::BTreeSet;
use std::fs::{create_dir_all, read_to_string, remove_dir_all};
use std::path::Path;
use std::{env, fs, io};

#[derive(Default, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ActionWhenExists {
    #[default]
    Terminate,
    Skip,
    Overwrite,
}

impl Into<ActionWhenRenamedFilePathExists> for ActionWhenExists {
    fn into(self) -> ActionWhenRenamedFilePathExists {
        match self {
            ActionWhenExists::Terminate => ActionWhenRenamedFilePathExists::Terminate,
            ActionWhenExists::Skip => ActionWhenRenamedFilePathExists::Skip,
            ActionWhenExists::Overwrite => ActionWhenRenamedFilePathExists::Overwrite,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct FilesEnvironmentConfig {
    pub environment_name: String,

    pub input_template: String,
    pub output_template: String,
    pub before: Vec<(String, Option<String>)>,
    pub after: Vec<String>,

    #[serde(default)]
    pub raise_error: bool,
    #[serde(default)]
    pub action_when_exists: ActionWhenExists,
}

#[derive(Debug)]
pub struct FilesEnvironment<'a> {
    root: String,
    files_environment_config: &'a FilesEnvironmentConfig,
}

impl<'a> FilesEnvironment<'a> {
    pub fn get_full_path(&self, path: &str) -> String {
        format!("{}/{}", &self.root, path)
    }

    pub fn new(files_environment_config: &'a FilesEnvironmentConfig) -> Result<Self, io::Error> {
        let root = env::temp_dir()
            .join("tfr-test-environment")
            .join(&files_environment_config.environment_name);
        create_dir_all(&root)?;

        for (path, _) in &files_environment_config.before {
            assert!(!path.is_empty());
            let path = format!("{}/{}", root.to_str().unwrap(), path);
            match path.chars().last().unwrap() {
                '/' => {
                    create_dir_all(path)?;
                }
                _ => {
                    create_dir_all(Path::new(&path).parent().unwrap())?;
                    fs::write(&path, &path)?;
                }
            }
        }
        Ok(Self {
            root: root.to_str().unwrap().to_string(),
            files_environment_config,
        })
    }

    pub fn is_after(&self) -> bool {
        for after_path in &self.files_environment_config.after {
            let after_path = self.get_full_path(after_path);

            let is_correct = match after_path.chars().last().unwrap() {
                '/' => Path::new(&after_path).is_dir(),
                _ => Path::new(&after_path).is_file(),
            };
            if !is_correct {
                return false;
            }
        }

        let all_renamed: BTreeSet<&String> = match self.files_environment_config.action_when_exists
        {
            ActionWhenExists::Overwrite => BTreeSet::from_iter(
                self.files_environment_config
                    .before
                    .iter()
                    .filter_map(|(_before, after)| after.as_ref()),
            ),
            _ => BTreeSet::default(),
        };

        for (before, after) in &self.files_environment_config.before {
            let full_before = self.get_full_path(before);

            let is_correct = match after {
                None => {
                    // must not be moved
                    if all_renamed.contains(&before) {
                        continue;
                    }
                    if !Path::new(&full_before).exists() {
                        false
                    } else if before.chars().last().unwrap() != '/' {
                        // is not directory
                        read_to_string(&full_before).unwrap_or("".to_string()) == full_before
                    } else {
                        true
                    }
                }
                Some(after) => {
                    // must be moved
                    let full_after = self.get_full_path(after);
                    !Path::new(&full_before).exists()
                        && read_to_string(&full_after).unwrap_or("".to_string()) == full_before
                }
            };
            if !is_correct {
                return false;
            }
        }

        true
    }
}

impl<'a> Drop for FilesEnvironment<'a> {
    fn drop(&mut self) {
        remove_dir_all(&self.root).unwrap();
    }
}
