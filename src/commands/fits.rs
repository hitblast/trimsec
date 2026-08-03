use crate::{
    cli::flags::Flags,
    commands::Runnable,
    core::{
        api::ApiClientManager,
        style::Style,
        time::{parse_duration, parse_time, time_in_day_after},
        utils::choose_or_grab_link,
        youtils::{get_youtube_api_key, get_youtube_id},
    },
};
use anyhow::{Result, bail};
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct FitsCmd {
    /// The URL, or link, for the YouTube video.
    #[arg(required_unless_present = "clip")]
    link: Option<String>,

    /// The budget duration string. By default uses the remaining time for the day.
    #[arg(short, long)]
    budget: Option<String>,

    /// Max amount of items to traverse in a playlist.
    #[arg(long, default_value = "0")]
    max_items: usize,
}

impl Runnable for FitsCmd {
    fn run(self, flags: &Flags, style: &Style) -> Result<()> {
        let key = get_youtube_api_key()?;
        let link = choose_or_grab_link(self.link, flags.clip)?;
        let manager = ApiClientManager::new(&key);
        let id = get_youtube_id(&link);

        let Some(id) = id else {
            bail!(
                "Not a valid YouTube URL! Only videos/embeds/shorts URLs are supported in the `yt` command."
            )
        };

        let (vid_total_duration, item_count) = manager
            .fetch_duration_from_id(&id, self.max_items)
            .map_err(|e| anyhow::anyhow!("Failed to fetch details from URL: {e}"))?;

        let message = {
            let status = if let Some(b) = &self.budget {
                let (limit_duration, _) = parse_duration(b)
                    .map_err(|e| anyhow::anyhow!("Failed to parse budget duration: {e}"))?;

                if limit_duration > vid_total_duration {
                    format!(
                        "{}Fits in budget!{}\n\nExtra time left: {}",
                        style.boldgreen(),
                        style.reset(),
                        parse_time(limit_duration - vid_total_duration)
                    )
                } else if limit_duration < vid_total_duration {
                    format!(
                        "{}Time overrun by {}!{}",
                        style.boldred(),
                        parse_time(vid_total_duration - limit_duration),
                        style.reset()
                    )
                } else {
                    "Duration match! Would finish on time.".to_string()
                }
            } else {
                let time_left = time_in_day_after(vid_total_duration);

                if time_left != 0.0 {
                    format!(
                        "{}Fits in day!{}\n\nTime left afterwards: {}",
                        style.boldgreen(),
                        style.reset(),
                        parse_time(time_left)
                    )
                } else {
                    format!(
                        "{}Content does not fit in the day.{}",
                        style.boldred(),
                        style.reset()
                    )
                }
            };

            format!("\n{status}\n(counted {item_count} videos)\n")
        };

        println!("{message}");
        Ok(())
    }
}
