use anyhow::{Result, bail};
use arboard::Clipboard;

pub fn choose_or_grab_link(link: Option<String>, no_clip: bool) -> Result<String> {
    let link = if link.is_none() && !no_clip {
        let mut c = Clipboard::new()?;
        let l = c.get_text().ok();

        if let Some(l) = l {
            l
        } else {
            bail!("No content found in clipboard.")
        }
    } else if let Some(l) = link {
        l
    } else {
        bail!("Link to YouTube object (video/playlist) is required. Aborting.")
    };

    Ok(link)
}
