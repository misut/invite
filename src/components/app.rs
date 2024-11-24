use crate::components::card::Card;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    // Build cool things ✌️
    rsx! {
        link { rel: "stylesheet", href: "./main.css" }
        Card {}
    }
}
