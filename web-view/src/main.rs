#![recursion_limit = "128"]

#[path = "./lib.rs"]
pub mod counter;

fn main() {
    yew::start_app::<counter::Model>();
}
