#![allow(non_snake_case)]

use components::app::App;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod components;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}
