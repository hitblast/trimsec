use crate::core::{
    api::ApiClientManager,
    time::{parse_duration, parse_time, time_in_day_after},
    utils::choose_or_grab_link,
    youtils::{get_youtube_api_key, get_youtube_id},
};
use anyhow::{Result, bail};
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct FitcheckCmd {
    /// The URL, or link, for the YouTube video. Defaults to: clipboard input.
    #[arg(short, long)]
    link: Option<String>,

    /// The budget duration string. Defaults to: none (the current day).
    #[arg(short, long)]
    budget: Option<String>,

    /// Max amount of items to traverse in a playlist. Default: 0 (uncapped).
    #[arg(long, default_value = "0")]
    max_items: usize,

    /// Chooses the best-fitting video for the duration.
    #[arg(short, long)]
    choose: bool,
}

impl FitcheckCmd {
    pub fn run(self, no_clip: bool) -> Result<()> {
        let key = get_youtube_api_key()?;
        let link = choose_or_grab_link(self.link, no_clip)?;
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

        let message = if let Some(b) = &self.budget {
            let (limit_duration, splits) = parse_duration(b)
                .map_err(|e| anyhow::anyhow!("Failed to parse budget duration: {e}"))?;

            let mut lines = vec![if limit_duration > vid_total_duration {
                format!(
                    "Fits in budget!\n\nExtra time left: {}",
                    parse_time(limit_duration - vid_total_duration)
                )
            } else if limit_duration < vid_total_duration {
                format!(
                    "Time overrun by {}!",
                    parse_time(vid_total_duration - limit_duration)
                )
            } else {
                "Duration match! Would finish on time.".to_string()
            }];

            if splits > 1 || item_count > 1 {
                lines.push(format!("(counted {item_count} videos and {splits} splits)"));
            }
            lines.join("\n")
        } else {
            let time_left = time_in_day_after(vid_total_duration);
            let mut lines = vec![if time_left != 0.0 {
                format!(
                    "Fits in day!\n\nTime left afterwards: {}",
                    parse_time(time_left)
                )
            } else {
                "Content does not fit in the day.".to_string()
            }];
            lines.push(format!("(counted {item_count} videos)"));
            lines.join("\n")
        };

        println!("{message}");
        Ok(())
    }
}
