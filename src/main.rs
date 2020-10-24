extern crate gdk;
extern crate gtk;
extern crate glib;
#[macro_use]
extern crate horrorshow;
extern crate pango;
extern crate pulldown_cmark;
extern crate sourceview;
extern crate tiny_keccak;
extern crate webkit2gtk;

pub mod preview;
pub mod state;
pub mod ui;
use ui::App;

fn main() {
    // println!("Hello, world!");

    App::new()
         .connect_events()
         .then_execute();
}
