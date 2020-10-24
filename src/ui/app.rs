use super::{Content, Notebook, Header, OpenDialog, OpenFolderDialog, SearchDialog};
use super::misc::*;
use super::save::save;
// use gdk::CONTROL_MASK;
// use gdk::enums::key;
use gtk;
use gtk::*;
use crate::preview::render;
use crate::ui::content::Source;
use crate::state::ActiveMetadata;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::{Arc, RwLock, Mutex};
// use std::sync::atomic::{AtomicBool, Ordering};
use std::fs::{self, DirEntry};
use webkit2gtk::*;

// upgrade weak reference or return
#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

impl App {
    pub fn new() -> App {
        if gtk::init().is_err() {
            eprintln!("Failed to initialize GTK Application");
            process::exit(1);
        }

        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new();

        window.set_titlebar(&header.container);
        window.set_title("Markdown editor");
        window.set_wmclass("md-editor", "Markdown editor");
        window.set_default_size(1200, 800);
        Window::set_default_icon_name("icon_name");

        window.add(&content.container);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App { window, header, content}
    }

    pub fn connect_events(self) -> ConnectApp {
        let current_file = Arc::new(RwLock::new(None));
        // let fullscreen = Arc::new(AtomicBool::new(false));

        {
            let save = &self.header.save;
            let save_as = &self.header.save_as;

            // self.editor_changed(current_file.clone(), &save);
            self.open_file(current_file.clone());
            self.open_dir_file(current_file.clone());
            self.open_folder(current_file.clone());
            self.search();
            // self.save_event(&save, &save, current_file.clone(), false);
            // self.save_event(&save, &save_as, current_file.clone(), true);
            // self.key_events(current_file, fullscreen);
        }
        ConnectApp(self)
    }

    fn open_file(&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>){
        // let editor = self.nest_container.content.source.buff.clone();
        // let preview = self.nest_container.content.preview.clone();

        let headerbar = self.header.container.clone();
        let notebook = self.content.notebook.notebook.clone();

        self.header.open.connect_clicked(move |_| {
            let open_dialog = OpenDialog::new({
                let lock = current_file.read().unwrap();
                if let Some(ref path) = *lock {
                    path.get_dir()
                } else {
                    None
                }
            });

            if let Some(new_file) = open_dialog.run() {
                if let Ok(mut file) = File::open(&new_file) {
                    let mut contents = String::new();
                    let _ = file.read_to_string(&mut contents);

                    set_title(&headerbar, &new_file);
                    if let Some(parent) = new_file.parent() {
                        let subtitle: &str = &parent.to_string_lossy();
                        headerbar.set_subtitle(subtitle);
                    }


                    if let Some(filename) = new_file.file_name() {
                        let title: &str = &filename.to_string_lossy();
                        let label = gtk::Label::new(title);

                        *current_file.write().unwrap() =
                            Some(ActiveMetadata::new(new_file, &contents.as_bytes()));

                        // create tab, notebook append_page, show_all
                        let close_image = gtk::Image::new_from_icon_name("window-close", IconSize::Button.into());
                        let button = gtk::Button::new();
                        button.set_relief(ReliefStyle::None);
                        button.set_focus_on_click(false);
                        button.add(&close_image);

                        let tab = gtk::Box::new(Orientation::Horizontal, 0);
                        tab.pack_start(&label, false, false, 0);
                        tab.pack_start(&button, false, false, 0);
                        tab.show_all();

                        let source = Source::new();

                        source.buff.set_text(&contents);

                        let index = notebook.append_page(&source.container, Some(&tab));
                        notebook.show_all();

                        let notebook_weak = notebook.downgrade();
                        button.connect_clicked(move |_| {
                            let notebook = upgrade_weak!(notebook_weak);
                            let index = notebook.page_num(&source.container)
                                                .expect("Couldn't get page_num from notebook");
                            notebook.remove_page(Some(index));
                        });

                        // notebook.create_tab(&title, label.upcast());
                        // notebook.lock().unwrap().create_tab(&title, label.upcast());

                        // println!("p {:?}", notebook.lock().unwrap());

                        // editor.set_text(&contents);
                        // preview.load_html(&render(&contents), None);
                    }
                };
            }
        });
        // println!("final {:?}", self.content.notebook);
    }

    fn open_dir_file(&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>){
        // let editor = self.nest_container.content.source.buff.clone();
        // let preview = self.nest_container.content.preview.clone();
        let headerbar = self.header.container.clone();
        let notebook = self.content.notebook.notebook.clone();

        let treeview = self.content.browser.treeview.clone();
        let left_selection = treeview.get_selection();

        left_selection.connect_changed(move |tree_selection| {
            let (left_model, iter) = tree_selection.get_selected().expect("Couldn't get selected");
            let mut value = left_model.get_value(&iter, 0).get::<String>().expect("Couldn't get value");

            // let mut value = String::new();
            // let tree_path = left_model.get_path(&iter).expect("Couldn't get path");
            // left_model.foreach( |left_model, tree_path, iter| {
            //     // value += &left_model.get_value(&iter, 0).get::<String>().expect("Couldn't get value");
            //     value.insert(0, '/');
            //     value.insert_str(0, &left_model.get_value(&iter, 0).get::<String>().expect("Couldn't get value"));
            //     true
            // });
            //
            // println!("path depth: {:?}", tree_path.get_depth());
            // println!("path get_indices: {:?}", tree_path.get_indices());
            println!("path value: {:?}", value);

            // let lock = current_file.read().unwrap();
            // if let Some(ref path) = *lock {
            //     path.get_dir();
            // } else {
            //     None
            // }

            let new_file = Path::new(&value).to_path_buf();
            println!("new open file path{:?}", new_file);

            if new_file.exists() {
                // println!("file path exists{:?}", new_file);
                if let Ok(mut file) = File::open(&new_file) {
                    let mut contents = String::new();
                    let _ = file.read_to_string(&mut contents);

                    set_title(&headerbar, &new_file);
                    if let Some(parent) = new_file.parent() {
                        let subtitle: &str = &parent.to_string_lossy();
                        headerbar.set_subtitle(subtitle);
                    }

                    if let Some(filename) = new_file.file_name() {
                        let title: &str = &filename.to_string_lossy();
                        let label = gtk::Label::new(title);

                        *current_file.write().unwrap() =
                        Some(ActiveMetadata::new(new_file, &contents.as_bytes()));

                        // create tab, notebook append_page, show_all
                        let close_image = gtk::Image::new_from_icon_name("window-close", IconSize::Button.into());
                        let button = gtk::Button::new();
                        button.set_relief(ReliefStyle::None);
                        button.set_focus_on_click(false);
                        button.add(&close_image);

                        let tab = gtk::Box::new(Orientation::Horizontal, 0);
                        tab.pack_start(&label, false, false, 0);
                        tab.pack_start(&button, false, false, 0);
                        tab.show_all();

                        let source = Source::new();

                        source.buff.set_text(&contents);

                        let index = notebook.append_page(&source.container, Some(&tab));
                        notebook.show_all();

                        let notebook_weak = notebook.downgrade();
                        button.connect_clicked(move |_| {
                            let notebook = upgrade_weak!(notebook_weak);
                            let index = notebook.page_num(&source.container)
                            .expect("Couldn't get page_num from notebook");
                            notebook.remove_page(Some(index));
                        });
                    }
                    // editor.set_text(&contents);
                    // preview.load_html(&render(&contents), None);
                } else {
                    println!("invalid path selected!");
                };
            }
        });
    }

    fn open_folder(&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>){
        let treestore = self.content.browser.treestore.clone();
        let liststore = self.content.browser.liststore.clone();
        // let editor = self.nest_container.content.source.buff.clone();
        // let preview = self.nest_container.content.preview.clone();
        let headerbar = self.header.container.clone();

        self.header.open_folder.connect_clicked(move |_| {
            let open_folder_dialog = OpenFolderDialog::new({
                let lock = current_file.read().unwrap();
                if let Some(ref path) = *lock {
                    path.get_dir()
                } else {
                    None
                }
            });

            // open_folder_dialog.run();
            if let Some(dirs) = open_folder_dialog.run() {
                let col_indices: [u32; 1] = [0];
                let project_dir = dirs[0].clone();
                let project_path = project_dir.as_path().parent().unwrap();

                treestore.clear();
                liststore.clear();

                for dir in dirs {
                    // add file to treestore & liststore
                    // let dir_name: [&dyn ToValue; 1] = [&dir.as_path().to_str()];

                    if let Ok(file_path) = dir.as_path().strip_prefix(&project_path){
                        let dir_name: [&dyn ToValue; 1] = [&file_path.to_str()];

                        let iter = treestore.insert_with_values(None, None, &[0], &dir_name);
                        liststore.set(&liststore.append(), &col_indices, &dir_name);

                        // if it's dir, also add sub_dir files to treestore
                        if dir.is_dir() {
                            let sub_dir = dir.clone();
                            if let Ok(entries) = fs::read_dir(dir){
                                for entry in entries {
                                    if let Ok(sub_dir_file) = entry {
                                        if let Ok(file_path) = sub_dir_file.path().as_path().strip_prefix(&sub_dir.as_path()){
                                            let dir_name: [&dyn ToValue; 1] = [&file_path.to_str()];
                                            treestore.insert_with_values(Some(&iter), None, &[0], &dir_name);
                                            liststore.set(&liststore.append(), &col_indices, &dir_name);
                                        }
                                    }
                                }
                            }
                        }
                    }

                }

            }
        });

    }

    fn search(&self){
        let liststore = self.content.browser.liststore.clone();
        self.header.search.connect_clicked(move |_| {
            // let search = SearchDialog::new();
            // search.show_all();

            let search_dialog = Dialog::new_with_buttons(Some("Hello!"),
                                                  Some(&Window::new(WindowType::Popup)),
                                                  gtk::DialogFlags::MODAL,
                                                  &[("Search", ResponseType::Yes)]);
            // search_dialog.show_all();

            // Create a title label
            let win_title = gtk::Label::new(None);
            win_title.set_markup("<big>Which country would you like to spend a holiday in?</big>");
            // let win_title = gtk::Label::new("Search box");

            // Create an EntryCompletion widget
            let completion_countries = gtk::EntryCompletion::new();
            // Use the first (and only) column available to set the autocompletion text
            completion_countries.set_text_column(0);
            // how many keystrokes to wait before attempting to autocomplete?
            completion_countries.set_minimum_key_length(1);
            // whether the completions should be presented in a popup window
            completion_countries.set_popup_completion(true);

            // Create a ListStore of items
            // These will be the source for the autocompletion
            // as the user types into the field
            // For a more evolved example of ListStore see src/bin/list_store.rs
            // let ls = create_list_model();
            // completion_countries.set_model(Some(&ls));
            completion_countries.set_model(Some(&liststore));

            let input_field = gtk::Entry::new();
            input_field.set_completion(Some(&completion_countries));

            let row = gtk::Box::new(gtk::Orientation::Vertical, 5);
            // row.add(&win_title);
            row.pack_start(&input_field, false, false, 10);

            let content_area = search_dialog.get_content_area();
            // search_dialog.add(&win_title);

            content_area.add(&win_title);
            content_area.add(&row);

            //show everything
            search_dialog.show_all();

        });
    }

    // fn save_event(
    //     &self,
    //     save_button: &Button,
    //     actual_button: &Button,
    //     current_file: Arc<RwLock<Option<ActiveMetadata>>>,
    //     save_as: bool,
    // ){
    //     let editor = self.nest_container.content.source.buff.clone();
    //     let headerbar = self.header.container.clone();
    //     let save_button = save_button.clone();
    //     actual_button.connect_clicked(
    //         move |_| save(&editor, &headerbar, &save_button, &current_file, save_as),
    //     );
    // }

    // fn editor_changed(
    //     &self,
    //     current_file: Arc<RwLock<Option<ActiveMetadata>>>,
    //     save_button: &Button,
    // ){
    //     let preview = self.nest_container.content.preview.clone();
    //     let save_button = save_button.clone();
    //     self.nest_container.content.source.buff.connect_changed(move |editor| {
    //         if let Some(markdown) = get_buffer(&editor) {
    //             preview.load_html(&render(&markdown), None);
    //             if let Some(ref current_file) = *current_file.read().unwrap() {
    //                 let has_same_sum = current_file.is_same_as(&markdown.as_bytes());
    //                 save_button.set_sensitive(!has_same_sum);
    //             }
    //         }
    //     });
    // }
}

pub struct ConnectApp(App);

impl ConnectApp {
    pub fn then_execute(self) {
        self.0.window.show_all();
        gtk::main();
    }
}
