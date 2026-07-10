use crate::error::Result;
use crate::os::PlatformConfig;
use std::io::ErrorKind;

/// A platform install command with template substitution and sudo wrapping.
#[derive(Debug)]
pub struct Command {
    /// Program to execute: `apt`, `sudo`, `brew`, etc.
    pub program: String,
    /// Arguments passed to the program.
    pub args: Vec<String>,
}

impl Command {
    /// Build a `Command` from a platform config, substituting `{package}` in the install template.
    ///
    /// If the config has `sudo: true`, the command is wrapped with `sudo <program> <args>`.
    pub fn from_config(config: &PlatformConfig, package: &str) -> Self {
        let template = &config.install;
        let cmd_str = template.replace("{package}", package);

        let parts = shlex::split(&cmd_str).unwrap_or_else(|| vec![cmd_str.clone()]);

        let (program, args) = if config.sudo {
            ("sudo".into(), {
                let mut a = vec![parts[0].clone()];
                a.extend(parts[1..].iter().cloned());
                a
            })
        } else {
            let mut iter = parts.into_iter();
            let prog = iter.next().unwrap_or_default();
            let a: Vec<String> = iter.collect();
            (prog, a)
        };

        Command { program, args }
    }

    /// Format the command as a human-readable string (for `--dry-run` display).
    pub fn to_display(&self) -> String {
        if self.program == "sudo" {
            format!("sudo {}", self.args.join(" "))
        } else {
            format!("{} {}", self.program, self.args.join(" "))
        }
    }

    /// Execute the command and wait for completion.
    ///
    /// Returns `Err(Error::Exec)` if the process fails to start or exits non-zero.
    pub fn run(&self) -> Result<()> {
        let status = std::process::Command::new(&self.program)
            .args(&self.args)
            .status()
            .map_err(|e| {
                if e.kind() == ErrorKind::NotFound {
                    crate::error::Error::ProgramNotFound(self.program.clone())
                } else {
                    crate::error::Error::Exec(format!("failed to execute: {e}"))
                }
            })?;

        if !status.success() {
            return Err(crate::error::Error::Exec(format!(
                "command exited with {status}"
            )));
        }

        Ok(())
    }
}
