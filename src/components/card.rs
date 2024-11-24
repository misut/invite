use dioxus::prelude::*;

pub fn Card() -> Element {
    rsx! {
        div { class: "container-vertical",
            img {
                id: "snowflake",
                src: "./snowflake.svg",
                style: "margin: 50px; width: 20vh; height: 20vh"
            }
            p { class: "font-title", "송년회 초대장" }
            p { class: "font-body", "0000년 00월 00일 00:00" }
            p { class: "font-subtitle", "장소" }
            Map {}
        }
    }
}

fn Map() -> Element {
    rsx! {
        div {
            id: "map",
            style: "border-radius: 10px; width: 30vh; height: 30vh;",
            script { src: "./map.js" }
        }
    }
}
