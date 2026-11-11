//! The part of arboard's API that `pdfcraft-ui-egui` uses, for platforms arboard doesn't support
//! (Android). The system clipboard can't be opened, so every caller takes its "no clipboard"
//! path; copy and paste inside the app still work through egui.
//! `pdfcraft-ui-egui` names this crate `arboard` there (`extern crate pdfcraft_no_clipboard as arboard`).
//!
//! ponytail: no system clipboard; Android's ClipboardManager over JNI when sharing with other
//! apps matters.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use std::borrow::Cow;
use std::path::Path;

/// Why the clipboard couldn't be used: always that there is none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ClipboardNotSupported,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("the system clipboard isn't available on this platform")
    }
}

impl std::error::Error for Error {}

/// An image as arboard passes it: RGBA, four bytes a pixel.
#[derive(Debug, Clone)]
pub struct ImageData<'a> {
    pub width: usize,
    pub height: usize,
    pub bytes: Cow<'a, [u8]>,
}

/// A clipboard that can never be opened.
#[derive(Debug)]
pub struct Clipboard(());

impl Clipboard {
    pub fn new() -> Result<Self, Error> {
        Err(Error::ClipboardNotSupported)
    }

    pub fn get_text(&mut self) -> Result<String, Error> {
        Err(Error::ClipboardNotSupported)
    }

    pub fn get_image(&mut self) -> Result<ImageData<'static>, Error> {
        Err(Error::ClipboardNotSupported)
    }

    pub fn set_image(&mut self, _image: ImageData<'_>) -> Result<(), Error> {
        Err(Error::ClipboardNotSupported)
    }

    pub fn set(&mut self) -> Set {
        Set(())
    }
}

/// arboard's builder for setting other formats.
#[derive(Debug)]
pub struct Set(());

impl Set {
    pub fn html<'a>(self, _html: impl Into<Cow<'a, str>>, _alt_text: Option<impl Into<Cow<'a, str>>>) -> Result<(), Error> {
        Err(Error::ClipboardNotSupported)
    }

    pub fn file_list(self, _paths: &[impl AsRef<Path>]) -> Result<(), Error> {
        Err(Error::ClipboardNotSupported)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_clipboard_is_never_available() {
        assert_eq!(Clipboard::new().err(), Some(Error::ClipboardNotSupported));
        assert!(Set(()).html("<b>a</b>", Some("a")).is_err());
    }
}
