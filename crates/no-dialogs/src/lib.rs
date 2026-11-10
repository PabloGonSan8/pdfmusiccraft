//! The part of rfd's API that `pdfcraft-ui-egui` uses, for platforms rfd doesn't support
//! (Android). Its dialogs never show anything themselves: they keep what the app asked for
//! (title, filters, file name, folder) so the app can show its own file browser instead, and
//! answer as if cancelled when awaited directly.
//! `pdfcraft-ui-egui` names this crate `rfd` there (`extern crate pdfcraft_no_dialogs as rfd`).

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use std::future::{Ready, ready};
use std::path::{Path, PathBuf};

/// A file dialog's settings, shown by the app's own browser.
#[derive(Clone, Debug, Default)]
pub struct AsyncFileDialog {
    title: String,
    filters: Vec<(String, Vec<String>)>,
    file_name: String,
    directory: Option<PathBuf>,
}

impl AsyncFileDialog {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    #[must_use]
    pub fn add_filter(mut self, name: impl Into<String>, extensions: &[impl ToString]) -> Self {
        self.filters.push((name.into(), extensions.iter().map(ToString::to_string).collect()));
        self
    }

    #[must_use]
    pub fn set_file_name(mut self, name: impl Into<String>) -> Self {
        self.file_name = name.into();
        self
    }

    #[must_use]
    pub fn set_directory<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.directory = Some(path.as_ref().to_path_buf());
        self
    }

    /// The title the app asked for.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// The filters, (name, extensions without the dot).
    pub fn filters(&self) -> &[(String, Vec<String>)] {
        &self.filters
    }

    /// The suggested file name (save dialogs).
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// The folder the dialog should start in.
    pub fn directory(&self) -> Option<&Path> {
        self.directory.as_deref()
    }

    pub fn pick_file(self) -> Ready<Option<FileHandle>> {
        ready(None)
    }

    pub fn pick_files(self) -> Ready<Option<Vec<FileHandle>>> {
        ready(None)
    }

    pub fn pick_folder(self) -> Ready<Option<FileHandle>> {
        ready(None)
    }

    pub fn save_file(self) -> Ready<Option<FileHandle>> {
        ready(None)
    }
}

/// A chosen file; never made here, as no dialog is ever answered.
#[derive(Clone, Debug)]
pub struct FileHandle(PathBuf);

impl FileHandle {
    pub fn path(&self) -> &Path {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialogs_keep_their_settings_and_are_cancelled_when_awaited() {
        let d = || AsyncFileDialog::new().set_title("Open").add_filter("PDF", &["pdf"]).set_file_name("a.pdf").set_directory("/");
        let s = d();
        assert_eq!((s.title(), s.file_name(), s.directory()), ("Open", "a.pdf", Some(Path::new("/"))));
        assert_eq!(s.filters(), &[("PDF".to_string(), vec!["pdf".to_string()])]);
        assert!(d().pick_file().into_inner().is_none());
        assert!(d().pick_files().into_inner().is_none());
        assert!(d().pick_folder().into_inner().is_none());
        assert!(d().save_file().into_inner().is_none());
    }
}
