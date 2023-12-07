use std::{
    io::{ErrorKind, Result},
    process::{Command, Stdio},
};

/// Determine if the provided program exists.
pub fn cmd_exists<T: AsRef<str>>(program: T) -> Result<()> {
    let mut cmd = Command::new(if cfg!(windows) { "where" } else { "sh" });

    let arg = {
        #[cfg(target_os = "linux")]
        {
            cmd.arg("-c"); // invocation flag
            format!("command -v {}", program.as_ref())
        }

        #[cfg(not(target_os = "linux"))]
        program.as_ref()
    };

    if cmd
        .arg(arg)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?
        .success()
    {
        Ok(())
    } else {
        Err(ErrorKind::NotFound.into())
    }
}
