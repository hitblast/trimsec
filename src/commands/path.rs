use crate::commands::{Ctx, Runnable};
use clap::Args;

#[derive(Debug, Args)]
pub struct PathCmd;

impl Runnable for PathCmd {
    fn run(self, ctx: &mut Ctx) -> anyhow::Result<()> {
        println!("{}", ctx.config()?.path().display());
        Ok(())
    }
}
