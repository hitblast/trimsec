use crate::{
    commands::{Ctx, Runnable},
    core::youtils::decide_youtube_key,
};
use clap::Args;

#[derive(Args, Debug)]
pub struct KeyShowCmd;

impl Runnable for KeyShowCmd {
    fn run(self, ctx: &mut Ctx) -> anyhow::Result<()> {
        println!(
            "{}",
            decide_youtube_key(ctx.config()?)
                .as_deref()
                .unwrap_or("not set")
        );
        Ok(())
    }
}
