use std::{env::VarError, io, process::ExitStatus};

use derive_more::{Display, Error, From};
use inquire::InquireError;

#[derive(Debug, Display, From, Error)]
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
    NoTemplates,
    #[display("non-existent template")]
    NonExistentTemplate,
}
