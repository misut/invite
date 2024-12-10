use crate::components::card::Card;
use crate::components::login::Login;
use dioxus::prelude::*;
use dioxus_sdk::storage::*;

#[component]
pub fn App() -> Element {
    // let logged_in = use_synced_storage::<LocalStorage, bool>("logged_in".to_string(), || false);
    let logged_as = use_singleton_persistent(|| "".to_string());
    rsx! {
        link { rel: "stylesheet", href: asset!("./assets/main.css") }
        if logged_as().is_empty() {
            Login { logged_as }
        } else {
            Card { ldap: logged_as }
        }
    }
}
