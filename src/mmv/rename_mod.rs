/// Possible behavior of `TemplateFileRenamer` when a new file path already exists
#[derive(Default, Eq, PartialEq)]
pub enum ActionWhenRenamedFilePathExists {
    #[default]
    Terminate,
    Skip,
    Overwrite,
}
