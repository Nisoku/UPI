use crate::error::Result;
use crate::os::PlatformConfig;

#[derive(Debug)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
}

impl Command {
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

    pub fn to_display(&self) -> String {
        if self.program == "sudo" {
            format!("sudo {}", self.args.join(" "))
        } else {
            format!("{} {}", self.program, self.args.join(" "))
        }
    }

    pub fn run(&self) -> Result<()> {
        let status = std::process::Command::new(&self.program)
            .args(&self.args)
            .status()
            .map_err(|e| crate::error::Error::Exec(format!("failed to execute: {e}")))?;

        if !status.success() {
            return Err(crate::error::Error::Exec(format!(
                "command exited with {status}"
            )));
        }

        Ok(())
    }
}
