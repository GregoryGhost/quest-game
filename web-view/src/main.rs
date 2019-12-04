#![recursion_limit = "256"]

//TODO: переместить в этот файл RootView
//TODO: выделить папку для компонентов, и для вьюх
#[path = "./root_view.rs"]
pub mod root;

pub fn main() {
    yew::start_app::<root::RootView>();
}
