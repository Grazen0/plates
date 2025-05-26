use std::{env::VarError, io, process::ExitStatus};

use derive_more::{Display, Error, From, IsVariant};
use inquire::InquireError;

pub type PlatesResult<T> = Result<T, PlatesError>;

#[derive(Debug, Display, From, Error, IsVariant)]
pub enum PlatesError {
    Inquire(InquireError),
    Io(io::Error),
    #[display("failed to get xdg directories")]
    Xdg(xdg::BaseDirectoriesError),
    #[display("failed to get environment variable \"{_0}\": {_1}")]
    EnvVar(String, VarError),
    #[display("failed to parse template config: {_0}")]
    Serde(serde_yaml::Error),
    #[display("failed to run a command with status {_0}")]
    Shell(#[error(not(source))] ExitStatus),
    #[display("no templates available")]
    NoTemplatesAvailable,
    #[display("non-existent template")]
    NonExistentTemplate,
    #[display("the given path is a file")]
    PathIsFile,
}
