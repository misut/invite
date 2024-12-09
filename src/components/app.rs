use crate::components::card::Card;
use crate::components::login::Login;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    // Build cool things ✌️
    let logged_in = use_signal(|| false);
    rsx! {
        link { rel: "stylesheet", href: "./main.css" }
        if !logged_in() {
            Login { logged_in }
        } else {
            Card {}
        }
    }
}
