use gtk::*;
use std::path::PathBuf;
use std::fs::{self, DirEntry};


pub struct OpenDialog(FileChooserDialog);

impl OpenDialog {
    pub fn new(path: Option<PathBuf>) -> OpenDialog {
        let open_dialog = FileChooserDialog::new(
            Some("Open"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Open,
        );
        open_dialog.add_button("Cancel", ResponseType::Cancel.into());
        open_dialog.add_button("Open", ResponseType::Ok.into());

        path.map(|p| open_dialog.set_current_folder(p));

        OpenDialog(open_dialog)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl Drop for OpenDialog {
    fn drop(&mut self)  { self.0.destroy(); }
}

pub struct OpenFolderDialog(FileChooserDialog);

impl OpenFolderDialog {
    pub fn new(path: Option<PathBuf>) -> OpenFolderDialog {
        let open_folder_dialog = FileChooserDialog::new(
            Some("Open Folder"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::SelectFolder,
        );
        open_folder_dialog.add_button("Cancel", ResponseType::Cancel.into());
        open_folder_dialog.add_button("Open", ResponseType::Ok.into());

        path.map(|p| open_folder_dialog.set_current_folder(p));

        OpenFolderDialog(open_folder_dialog)
    }

    pub fn run(&self) -> Option<Vec<PathBuf>>{
        if self.0.run() == ResponseType::Ok.into() {
            let mut vec: Vec<PathBuf> = Vec::new();
            let mut vec_files: Vec<PathBuf> = Vec::new();
            if let Some(dir) = self.0.get_filename(){
                let folder_dir = dir.clone();
                if let Ok(entries) = fs::read_dir(dir){
                    for entry in entries{
                        if let Ok(entry) = entry {
                            // println!("{:?}", entry.path());
                            // if entry.path().is_dir() {
                            //     if let Ok(path) = entry.path().strip_prefix(&folder_dir) {
                            //         vec.push(path.to_path_buf());
                            //     }
                            // } else {
                            //     if let Ok(path) = entry.path().strip_prefix(&folder_dir) {
                            //         vec_files.push(path.to_path_buf());
                            //     }
                            // }
                            if entry.path().is_dir() {
                                vec.push(entry.path());
                            } else {
                                vec_files.push(entry.path());
                            }
                        }
                    }
                    vec.append(&mut vec_files);
                }
            }
            Some(vec)
        } else {
            None
        }
    }

}

impl Drop for OpenFolderDialog {
    fn drop(&mut self)  { self.0.destroy(); }
}

pub struct SaveDialog(FileChooserDialog);

impl SaveDialog {
    pub fn new(path: Option<PathBuf>) -> SaveDialog {
        let save_dialog = FileChooserDialog::new(
            Some("Save"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Save,
        );

        save_dialog.add_button("Cancel", ResponseType::Cancel.into());
        save_dialog.add_button("Save", ResponseType::Ok.into());

        path.map(|p| save_dialog.set_current_folder(p));

        SaveDialog(save_dialog)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl Drop for SaveDialog {
    fn drop(&mut self) { self.0.destroy(); }
}


struct Data {
    description: String,
}

fn create_list_model() -> gtk::ListStore {
    let col_types: [gtk::Type; 1] = [gtk::Type::String];

    let data: [Data; 4] = [
        Data {
            description: "France".to_string(),
        },
        Data {
            description: "Italy".to_string(),
        },
        Data {
            description: "Sweden".to_string(),
        },
        Data {
            description: "Switzerland".to_string(),
        },
    ];
    let store = gtk::ListStore::new(&col_types);
    let col_indices: [u32; 1] = [0];
    for d in data.iter() {
        let values: [&dyn ToValue; 1] = [&d.description];
        store.set(&store.append(), &col_indices, &values);
    }
    store
}

pub struct SearchDialog(Dialog);

impl SearchDialog{
    pub fn new() -> SearchDialog {
        let search_dialog = Dialog::new_with_buttons(Some("Hello!"),
                                              Some(&Window::new(WindowType::Popup)),
                                              gtk::DialogFlags::MODAL,
                                              &[("No", ResponseType::No),
                                                ("Yes", ResponseType::Yes),
                                                ("Custom", ResponseType::Other(0))]);

        println!("SearchDialog");
        // // Create a title label
        // // let win_title = gtk::Label::new(None);
        // // win_title.set_markup("<big>Which country would you like to spend a holiday in?</big>");
        // let win_title = gtk::Label::new("Search box");
        //
        // // Create an EntryCompletion widget
        // let completion_countries = gtk::EntryCompletion::new();
        // // Use the first (and only) column available to set the autocompletion text
        // completion_countries.set_text_column(0);
        // // how many keystrokes to wait before attempting to autocomplete?
        // completion_countries.set_minimum_key_length(1);
        // // whether the completions should be presented in a popup window
        // completion_countries.set_popup_completion(true);
        //
        // // Create a ListStore of items
        // // These will be the source for the autocompletion
        // // as the user types into the field
        // // For a more evolved example of ListStore see src/bin/list_store.rs
        // let ls = create_list_model();
        // completion_countries.set_model(Some(&ls));
        //
        // let input_field = gtk::Entry::new();
        // input_field.set_completion(Some(&completion_countries));
        //
        // let row = gtk::Box::new(gtk::Orientation::Vertical, 5);
        // // row.add(&win_title);
        // row.pack_start(&input_field, false, false, 10);
        //
        // let content_area = search_dialog.get_content_area();
        // // search_dialog.add(&win_title);
        //
        // content_area.add(&win_title);
        // // content_area.add(&row);

        // show everything
        // search_dialog.show_all();
        // search_dialog.show_all();

        SearchDialog(search_dialog)
    }

    pub fn show_all(&self) {
        self.0.show_all();
    }

    // pub fn run(&self) -> Option<PathBuf> {
    //     if self.0.run() == ResponseType::Ok.into() {
    //         self.0.get_filename()
    //     } else {
    //         None
    //     }
    // }
}

// impl ShowAll for SearchDialog {
//     fn show_all(&self) { self.0.show_all(); }
// }

impl Drop for SearchDialog {
    fn drop(&mut self)  { self.0.destroy(); }
}
