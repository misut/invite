use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct LoginProps {
    logged_in: Signal<bool>,
}

impl LoginProps {
    fn onlogin(&mut self) {
        self.logged_in.set(true);
    }
}

pub fn Login(mut props: LoginProps) -> Element {
    rsx! {
        form {
            class: "form-login",
            onsubmit: move |_event| {
                props.onlogin();
            },

            p { class: "font-title margin-large", "?" }

            input {
                class: "font-input user-input",
                r#type: "text",
                name: "ldap",
                placeholder: "👤 LDAP"
            }

            input {
                class: "font-input user-input",
                r#type: "password",
                name: "code",
                placeholder: "🔒 Invitation Code"
            }

            input { class: "font-submit submit-button", r#type: "submit" }
        }
    }
}
