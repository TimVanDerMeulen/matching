//mod matching;
//mod ui;
//
//fn main() {
//    yew::start_app::<Model>();
//}
use crate::matching::process;
use std::fs;

mod matching;

fn main() {
    let data = fs::read_to_string("res/concept.json").expect("Unable to read file");
    process(data);
}
