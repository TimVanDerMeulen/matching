extern crate console_error_panic_hook;
use std::panic;
extern crate log;
extern crate yew;

use log::Level;

mod matching;
mod ui;

use ui::ui::BaseModel;

fn main() {
    console_log::init_with_level(Level::Debug);
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    yew::start_app::<BaseModel>();
}
