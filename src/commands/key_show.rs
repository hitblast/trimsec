use crate::core::{context::Ctx, youtils::decide_youtube_key};
use clap::Args;

#[derive(Args, Debug)]
pub struct KeyShowCmd;

impl KeyShowCmd {
    pub fn run(self, ctx: &mut Ctx) -> anyhow::Result<()> {
        println!(
            "{}",
            decide_youtube_key(ctx.config()?)
                .as_deref()
                .unwrap_or("not set")
        );
        Ok(())
    }
}
