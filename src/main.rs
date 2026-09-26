use anyhow::Result;
use trimsec::args::get_cur_cmd;

fn main() -> Result<()> {
    let (args, cmd) = get_cur_cmd()?;
    cmd.run(args)?;

    Ok(())
}
