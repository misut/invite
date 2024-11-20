use dioxus::prelude::*;

#[component]
pub fn Entrance() -> Element {
    rsx! {
        div {
            p { "?" }
        }
    }
}
