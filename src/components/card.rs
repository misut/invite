use dioxus::prelude::*;

pub fn Card() -> Element {
    rsx! {
        div { class: "container-vertical",
            img {
                id: "snowflake",
                class: "margin-large",
                src: "./snowflake.svg",
                style: "width: 20vh; height: 20vh"
            }
            p { class: "font-title margin-large", "송년회 초대장" }
            p { class: "font-body margin-small", "0000년 00월 00일 00:00" }
            p { class: "font-body margin-small", "경기도 성남시 분당구 00로 00" }
            p { class: "font-subtitle margin-large", "장소" }
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
