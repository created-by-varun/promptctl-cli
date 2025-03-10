use crate::error::Result;

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(text)?;
    Ok(())
}

pub fn open_url(url: &str) -> Result<()> {
    open::that(url)?;
    Ok(())
}
