#![recursion_limit = "128"]

#[path = "./lib.rs"]
pub mod counter;

#[path = "./file_upload_sample.rs"]
pub mod file_upload;

fn main() {
    // yew::start_app::<counter::SceneModel>();
    yew::start_app::<file_upload::Model>();
}
