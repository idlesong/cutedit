use gtk::*;

pub struct Header {
    pub container: HeaderBar,
    pub open: Button,
    pub open_folder: Button,
    pub search: Button,
    pub save: Button,
    pub save_as: Button,
}

impl Header {
    pub fn new() -> Header {
        let container = HeaderBar::new();
        container.set_title("Markdown editor");
        container.set_show_close_button(true);

        let open = Button::new_with_mnemonic("_Open");
        let open_folder = Button::new_with_mnemonic("_Open Folder");
        let search = Button::new_with_mnemonic("_Search");
        let save = Button::new_with_mnemonic("_Save");
        let save_as = Button::new_with_mnemonic("Save _As");

        container.pack_start(&open);
        container.pack_start(&open_folder);
        container.pack_start(&search);
        container.pack_end(&save);
        container.pack_end(&save_as);

        Header {container, open, open_folder, search, save, save_as}

    }
}
