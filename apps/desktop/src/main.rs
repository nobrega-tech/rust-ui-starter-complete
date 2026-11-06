mod app;
mod navigation;
mod pages;

use dioxus::prelude::*;

fn main() {
    dioxus::launch(app::App);
}
