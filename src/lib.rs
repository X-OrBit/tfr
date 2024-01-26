#![forbid(unsafe_code)]

mod mmv;

pub use mmv::{ActionWhenRenamedFilePathExists, TemplateFileRenamer, TfrError};
