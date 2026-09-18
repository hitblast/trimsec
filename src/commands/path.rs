use crate::commands::Ctx;
use clap::Args;

#[derive(Debug, Args)]
pub struct PathCmd;

impl PathCmd {
    pub fn run(self, ctx: &mut Ctx) -> anyhow::Result<()> {
        println!("{}", ctx.config()?.path().display());
        Ok(())
    }
}
