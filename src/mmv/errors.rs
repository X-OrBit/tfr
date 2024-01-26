use crate::mmv::file_utils::TemplateError;
use crate::mmv::TfrError::IncorrectInputTemplate;

/// Common Template File Renamer Errors
///
/// - `IncorrectInputTemplate` and `IncorrectOutputTemplate` occur when the passed templates are incorrect.
/// - `ExistingPath` occurs when the renaming mod is terminated if an existing path is found or existing path
///   is something except file
/// - Other errors ([std::error::Error](std::error::Error)) saved in 'StdError'. It was expected that only errors from
/// [fs::rename](std::fs::rename) can be occur here.
#[derive(Debug)]
pub enum TfrError {
    IncorrectInputTemplate(&'static str),
    IncorrectOutputTemplate(&'static str),
    ExistingPath(/*path=*/ String, /*is_file=*/ bool),
    StdError(Box<dyn std::error::Error>),
}

impl From<TemplateError> for TfrError {
    fn from(template_err: TemplateError) -> Self {
        match template_err {
            TemplateError::AsteriskInDirectory => {
                IncorrectInputTemplate("Found asterisks in directory of input template")
            }
            TemplateError::DoubleAsterisk => {
                IncorrectInputTemplate("Found double asterisk in input template")
            }
        }
    }
}

impl<StdError> From<StdError> for TfrError
where
    StdError: std::error::Error + 'static,
{
    fn from(template_err: StdError) -> Self {
        Self::StdError(template_err.into())
    }
}
