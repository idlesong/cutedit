use super::SaveDialog;
use super::misc::*;
use gtk::*;
use sourceview::*;
use crate::state::ActiveMetadata;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::RwLock;

pub enum SaveAction {
    New(ActiveMetadata),
    Saved,
    Canceled,
}

pub fn save(
    editor: &Buffer,
    headerbar: &HeaderBar,
    save: &Button,
    current_file: &RwLock<Option<ActiveMetadata>>,
    save_as: bool,
) {
    if let Some(text) = get_buffer(editor) {
        let result = if save_as {
            write_data(None, text.as_bytes())
        } else {
            write_data(current_file.read().unwrap().as_ref(), text.as_bytes())
        };

        match result {
            Ok(SaveAction::New(file)) => {
                set_title(&headerbar, file.get_path());
                if let Some(parent) = file.get_dir() {
                    let subtitle: &str = &parent.to_string_lossy();
                    headerbar.set_subtitle(subtitle);
                }
                let mut current_file = current_file.write().unwrap();
                *current_file = Some(file);
                save.set_sensitive(false);
            },
            Ok(SaveAction::Saved) => {
                if let Some(ref mut current_file) = *current_file.write().unwrap() {
                    current_file.set_sum(&text.as_bytes());
                    save.set_sensitive(false);
                }
            },
            _ => (),
        }
    }
}

fn write_data(path: Option<&ActiveMetadata>, data: &[u8]) -> io::Result<SaveAction> {
    if let Some(path) = path {
        let mut file =
            OpenOptions::new().create(true).write(true).truncate(true).open(path.get_path())?;
            file.write_all(&data)?;
            return Ok(SaveAction::Saved);
    }

    let save_dialog = SaveDialog::new(None);
    if let Some(new_path) = save_dialog.run() {
        let mut file =
            OpenOptions::new().create(true).write(true).truncate(false).open(&new_path)?;
        file.write_all(data)?;
        Ok(SaveAction::New(ActiveMetadata::new(new_path, data)))
    } else {
        Ok(SaveAction::Canceled)
    }
}
