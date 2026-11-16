use std::ffi::{OsStr, OsString};
use std::process::Command;

/// Utility for running `php-config` commands.
#[derive(Debug)]
pub struct PhpConfigRunner {
    /// Actual Command object.
    inner: Command,
    /// Arguments added to the command, for use in debugging.
    args: Vec<OsString>,
}

impl PhpConfigRunner {
    /// Create a new command runner.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Command::new("php-config"),
            args: vec![],
        }
    }

    /// Add an argument.
    pub fn arg<S>(&mut self, arg: S) -> &mut Self
    where
        S: AsRef<OsStr>,
    {
        self.args.push(arg.as_ref().to_os_string());
        let _: &mut Command = self.inner.arg(arg);
        self
    }

    /// Run the command and return the output, which must be UTF-8.
    ///
    /// # Panics
    ///
    /// Panics if the command fails or returns output that is not valid UTF-8.
    #[expect(clippy::panic, reason = "Panic is at build time")]
    pub fn run(&mut self) -> String {
        let output = self.inner.output().unwrap_or_else(|err| {
            panic!(
                "php-config {} should succeed, got {err}",
                self.args
                    .iter()
                    .map(|arg| arg.display().to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        });
        let stdout_full = std::str::from_utf8(&output.stdout).unwrap_or_else(|err| {
            panic!(
                "php-config {} should result in UTF-8, got {err}",
                self.args
                    .iter()
                    .map(|arg| arg.display().to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        });
        stdout_full.trim().to_owned()
    }
}

impl Default for PhpConfigRunner {
    fn default() -> Self {
        Self::new()
    }
}
