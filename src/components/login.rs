use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct LoginProps {
    logged_as: Signal<String>,
}

impl LoginProps {
    fn check_login(&mut self, icode: &str, ldap: &str) {
        let expect_icode: &str = option_env!("INVITATION_CODE")
            .or(Some("test"))
            .expect("$INVITATION_CODE not set");
        let expect_ldaps: Vec<&str> = option_env!("LDAP_LIST")
            .or(Some("foo.one,foo.two,foo.three,foo.four"))
            .expect("$LDAP_LIST not set")
            .split(",")
            .collect();

        if expect_icode == icode && expect_ldaps.contains(&ldap) {
            self.logged_as.set(ldap.to_string());
        }
    }
}

pub fn Login(mut props: LoginProps) -> Element {
    let mut icode = use_signal(|| "".to_string());
    let mut ldap = use_signal(|| "".to_string());
    rsx! {
        form {
            class: "form-login",
            onsubmit: move |_event| {
                props.check_login(&icode(), &ldap());
            },

            p { class: "font-title margin-large", "?" }

            input {
                class: "font-input user-input",
                r#type: "text",
                name: "ldap",
                oninput: move |event| ldap.set(event.value()),
                placeholder: "👤 LDAP"
            }

            input {
                class: "font-input user-input",
                r#type: "password",
                name: "icode",
                oninput: move |event| icode.set(event.value()),
                placeholder: "🔒 Invitation Code"
            }

            input { class: "font-submit submit-button", r#type: "submit" }
        }
    }
}
