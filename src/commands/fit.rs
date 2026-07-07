use arboard::Clipboard;
use clap::Args;

use anyhow::{Result, bail};

use crate::core::{
    api::ApiClientManager,
    time::{parse_duration, parse_time, time_in_day_after},
    youtils::{get_youtube_api_key, get_youtube_id},
};

#[derive(Debug, Default, Args)]
pub struct FitCmd {
    /// The URL, or link, for the YouTube video.
    #[arg(short, long)]
    link: Option<String>,

    /// The budget duration string. Defaults to: none (the current day).
    #[arg(short, long)]
    budget: Option<String>,

    /// Max amount of items to traverse in a playlist. Default: 0 (uncapped).
    #[arg(long, default_value = "0")]
    max_items: usize,

    /// Disable grabbing links from clipboard.
    #[arg(short, long)]
    noclip: bool,

    /// Chooses the best-fitting video for the duration.
    #[arg(short, long)]
    choose: bool,
}

impl FitCmd {
    pub fn run(self) -> Result<()> {
        let key = match get_youtube_api_key() {
            Some(key) => key,
            None => bail!(
                "Missing environment variable: TRIMSEC_YOUTUBE_KEY; read README.md for more information."
            ),
        };

        let link = if self.link.is_none() && !self.noclip {
            let mut c = Clipboard::new()?;
            let l = c.get_text().ok();

            if let Some(l) = l {
                l
            } else {
                bail!("No content found in clipboard.")
            }
        } else if let Some(l) = self.link {
            l
        } else {
            bail!("Link to YouTube object (video/playlist) is required. Aborting.")
        };

        let manager = ApiClientManager::new(&key);
        let id = get_youtube_id(&link);
        if let Some(id) = id {
            match manager.fetch_duration_from_id(&id, self.max_items) {
                Ok((vid_total_duration, item_count)) => {
                    if let Some(b) = &self.budget {
                        match parse_duration(b) {
                            Ok((limit_duration, splits)) => {
                                if limit_duration > vid_total_duration {
                                    println!(
                                        "Fits in budget!\n\nExtra time left: {}",
                                        parse_time(limit_duration - vid_total_duration)
                                    );
                                } else if limit_duration < vid_total_duration {
                                    println!(
                                        "Time overrun by {}!",
                                        parse_time(vid_total_duration - limit_duration)
                                    )
                                } else {
                                    println!("Duration match! Would finish on time.")
                                }

                                if splits > 1 || item_count > 1 {
                                    println!("(counted {item_count} videos and {splits} splits)")
                                }
                            }
                            Err(e) => bail!("Failed to parse budget duration: {e}"),
                        }
                    } else {
                        let time_left = time_in_day_after(vid_total_duration);

                        if time_left != 0.0 {
                            println!(
                                "Fits in day!\n\nTime left afterwards: {}",
                                parse_time(time_left)
                            )
                        } else {
                            println!("Content does not fit in the day.")
                        }

                        println!("(counted {item_count} videos)")
                    }
                }
                Err(e) => bail!("Failed to fetch details from URL: {e}"),
            }
        } else {
            bail!(
                "Not a valid YouTube URL! Only videos/embeds/shorts URLs are supported in the `yt` command."
            )
        }

        Ok(())
    }
}
