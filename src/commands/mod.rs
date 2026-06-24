pub mod trim;
pub mod yt;

use anyhow::Result;

pub trait Runnable {
    fn run(&self) -> Result<()>;
}
