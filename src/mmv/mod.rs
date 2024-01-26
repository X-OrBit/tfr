mod errors;
mod file_utils;
mod rename_mod;
mod template_applier;

pub use errors::TfrError;
pub use rename_mod::ActionWhenRenamedFilePathExists;

use std::fs;
use std::fs::create_dir_all;
use std::path::Path;

use crate::ActionWhenRenamedFilePathExists::Overwrite;
use template_applier::apply_template;

type CallbackHandler<'ch> = dyn Fn(usize, usize, Option<&str>, Option<&str>) + 'ch;

/// Provides template file paths renaming.
///
/// It is possible to use one of the 3 renaming mods [ActionWhenRenamedFilePathExists](ActionWhenRenamedFilePathExists)
///
/// While `TemplateFileRenamer` renames matched files, a custom
/// [CallbackHandler](TemplateFileRenamer::set_callback_handler) is called
///
/// # Examples
/// ```
/// use tfr::{ActionWhenRenamedFilePathExists, TemplateFileRenamer, TfrError};
/// let tfr = TemplateFileRenamer::new(ActionWhenRenamedFilePathExists::Terminate);
/// let _ = tfr.rename("path/to/before_*.*", "path/to/after_#1.#2");
/// ```
#[derive(Default)]
pub struct TemplateFileRenamer<'ch> {
    rename_mod: ActionWhenRenamedFilePathExists,
    callback_handler: Option<Box<CallbackHandler<'ch>>>,
}

impl<'ch> TemplateFileRenamer<'ch> {
    pub fn new(rename_mod: ActionWhenRenamedFilePathExists) -> Self {
        Self {
            rename_mod,
            callback_handler: None,
        }
    }

    /// # Signature
    /// dyn Fn(processed: usize, total: usize, old_filepath: Option<&str>, new_filepath: Option<&str>)
    ///
    /// # Description
    /// [TemplateFileRenamer](TemplateFileRenamer) provides custom user defined renaming callback handler
    ///
    /// When renaming is just started, `TFR` call `callback_handler` with arguments `processed=0` and `total` equal to count
    /// of files that matched `input_file_template`. `old_filepath` and `new_filepath` is None.
    ///
    /// When `processed` file was renamed, `TFR` call `callback_handler` with corresponding arguments.
    ///
    /// # Example
    /// ```
    /// let callback_handler = |processed: usize, total: usize, old_filepath: Option<&str>, new_filepath: Option<&str>| {
    ///     if processed == 0 {
    ///         println!("Renaming started")
    ///     } else {
    ///         println!("{} of {} already renamed", processed, total)
    ///     };
    ///  };
    /// ```
    pub fn set_callback_handler(
        &mut self,
        callback_handler: impl Fn(usize, usize, Option<&str>, Option<&str>) + 'ch,
    ) {
        self.callback_handler = Some(Box::new(callback_handler))
    }

    fn start(&self, total: usize) {
        if self.callback_handler.is_some() {
            (self.callback_handler.as_ref().unwrap())(0, total, None, None)
        }
    }

    fn callback(&self, current: usize, total: usize, old_filepath: &str, new_filepath: &str) {
        if self.callback_handler.is_some() {
            (self.callback_handler.as_ref().unwrap())(
                current,
                total,
                Some(old_filepath),
                Some(new_filepath),
            )
        }
    }

    /// Returns Ok(()) if all files matching the template have been successfully renamed
    ///
    /// Returns Err([TfrError](TfrError)) if any error occurred during renaming
    pub fn rename(
        &self,
        input_file_template: &str,
        output_file_template: &str,
    ) -> Result<(), TfrError> {
        let applied_new_filepaths =
            apply_template(input_file_template, output_file_template, &self.rename_mod)?;

        self.start(applied_new_filepaths.len());

        for (idx, (first, second)) in applied_new_filepaths.iter().enumerate() {
            if let Some(parent) = Path::new(second).parent() {
                create_dir_all(parent)?;
            }

            if self.rename_mod == Overwrite && Path::new(second).is_file() {
                fs::remove_file(second)?;
            }
            fs::rename(first, second)?;
            self.callback(idx + 1, applied_new_filepaths.len(), first, second);
        }
        Ok(())
    }
}
