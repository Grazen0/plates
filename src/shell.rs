use std::{
    ffi::OsStr,
    io::{self, Write},
    process::{Command, Output, Stdio},
};

pub fn create_shell_command(command: impl AsRef<OsStr>) -> Command {
    let mut cmd = Command::new("/bin/sh");
    cmd.arg("-c").arg(command).stdout(Stdio::piped());
    cmd
}

pub fn run_command_with_stdin(
    command: impl AsRef<OsStr>,
    input: impl AsRef<[u8]>,
) -> io::Result<Output> {
    let mut cmd = create_shell_command(command);

    let mut process = cmd.stdin(Stdio::piped()).spawn()?;
    process.stdin.take().unwrap().write_all(input.as_ref())?;

    process.wait_with_output()
}
