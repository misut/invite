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

            label { class: "input-label", r#for: "ldap", "LDAP" }
            input { class: "user-input", r#type: "text", name: "ldap" }

            label { class: "input-label", r#for: "code", "Invitation Code" }
            input { class: "user-input", r#type: "password", name: "code" }

            input { r#type: "submit" }
        }
    }
}
