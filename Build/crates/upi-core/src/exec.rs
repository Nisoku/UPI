use crate::error::Result;

pub struct Command {
    pub program: String,
    pub args: Vec<String>,
}

pub fn run(_cmd: &Command) -> Result<()> {
    Err(crate::error::Error::Exec(
        "execution not implemented".into(),
    ))
}
