use gtk::*;
use pango::*;
use sourceview::*;
use webkit2gtk::*;

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{IconSize, Orientation, ReliefStyle, Widget};

use std::env::args;

// upgrade weak reference or return
// #[macro_export]
// macro_rules! upgrade_weak {
//     ($x:ident, $r:expr) => {{
//         match $x.upgrade() {
//             Some(o) => o,
//             None => return $r,
//         }
//     }};
//     ($x:ident) => {
//         upgrade_weak!($x, ())
//     };
// }

pub struct Content {
    pub container: Paned,
    pub browser: Browser,
    pub notebook: Notebook,
    pub preview: WebView,
}

pub struct Browser {
    pub container: ScrolledWindow,
    pub treeview: TreeView,
    pub treestore: TreeStore,
    pub liststore: ListStore,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Notebook {
    pub notebook: gtk::Notebook,
    pub tabs: Vec<gtk::Box>
}

pub struct Source {
    pub container: ScrolledWindow,
    pub view: View,
    pub buff: Buffer,
}

impl Content {
    pub fn new() -> Content {
        let container = Paned::new(Orientation::Horizontal);
        let browser = Browser::new();
        let notebook = Notebook::new();
        // let source = Source::new();

        // // ================
        // for i in 1..4 {
            // let title = format!("sheet {}", 1);
            // let label = gtk::Label::new(&*title);
            // notebook.create_tab(&title, label.upcast());
        // }
        //
        // println!("init {:?}", notebook);

        let context = WebContext::get_default().unwrap();
        let preview = WebView::new_with_context(&context);

        container.pack1(&browser.container, true, true);
        container.pack2(&notebook.notebook, true, true);

        browser.container.set_size_request(100, -1);
        notebook.notebook.set_size_request(600, -1);
        // container.pack1(&source.container, true, true);
        // container.pack2(&preview, true, true);

        // source.container.set_size_request(100, -1);
        // preview.set_size_request(100, -1);

        Content { container, browser, notebook, preview }
    }
}

impl Browser {
    fn new() -> Browser {
        let treeview = TreeView::new();
        let treestore = TreeStore::new(&[String::static_type()]);

        // create liststore for search_dialog
        let data = [ "France".to_string(), "Italy".to_string(), "Sweden".to_string(),
                     "Switzerland".to_string(), "Shanghai".to_string()];

        let liststore = gtk::ListStore::new(&[String::static_type()]);

        let col_indices: [u32; 1] = [0];
        for d in data.iter() {
            let values: [&dyn ToValue; 1] = [&d];
            liststore.set(&liststore.append(), &col_indices, &values);
        }


        treeview.set_model(Some(&treestore));
        treeview.set_headers_visible(false);
        append_text_column(&treeview);

        // let view = View::new_with_buffer(&buff);
        let container = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        container.add(&treeview);

        Browser {container, treeview, treestore, liststore}
    }
}

fn append_text_column(tree: &TreeView) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

impl Source {
    pub fn new() -> Source {
        let buff = Buffer::new(None::<&gtk::TextTagTable> );
        let view = View::new_with_buffer(&buff);
        let container = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        container.add(&view);

        configure_source_view(&view, &buff);

        Source {container, view, buff}
    }
}

fn configure_source_view(view: &View, buff: &Buffer) {

    WidgetExt::override_font(view, &FontDescription::from_string("monospace"));

    LanguageManager::new()
    .get_language("markdown")
    .map(|markdown| buff.set_language(&markdown));

    let manager = StyleSchemeManager::new();
    manager
    .get_scheme("Builder")
    .or(manager.get_scheme("Classic"))
    .map(|theme| buff.set_style_scheme(&theme));

    view.set_show_line_numbers(true);
    // view.set_monospace(true);
    view.set_insert_spaces_instead_of_tabs(true);
    view.set_indent_width(4);
    // view.set_smart_backspace(true);
    view.set_right_margin(100);
    view.set_left_margin(10);
    view.set_show_right_margin(true);
    // view.set_background_pattern(BackgroundPatternType::Grid);
}

impl Notebook {
    pub fn new() -> Notebook {
        Notebook {
            notebook: gtk::Notebook::new(),
            tabs: Vec::new()
        }
    }

    pub fn create_tab(&mut self, title: &str, widget: Widget) -> u32 {
        let close_image = gtk::Image::new_from_icon_name("window-close",
                                                         IconSize::Button.into());
        let button = gtk::Button::new();
        let label = gtk::Label::new(title);
        let tab = gtk::Box::new(Orientation::Horizontal, 0);

        button.set_relief(ReliefStyle::None);
        button.set_focus_on_click(false);
        button.add(&close_image);

        tab.pack_start(&label, false, false, 0);
        tab.pack_start(&button, false, false, 0);
        tab.show_all();

        let index = self.notebook.append_page(&widget, Some(&tab));

        // let notebook_weak = self.notebook.downgrade();
        // button.connect_clicked(move |_| {
        //     let notebook = upgrade_weak!(notebook_weak);
        //     let index = notebook.page_num(&widget)
        //                         .expect("Couldn't get page_num from notebook");
        //     notebook.remove_page(Some(index));
        // });

        self.tabs.push(tab);

        index
    }
}
