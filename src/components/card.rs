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
            p { class: "font-body margin-small", "2024년 12월 18일 18:00" }
            p { class: "font-body margin-small", "경기도 성남시 분당구 00로 00" }
            p { class: "font-comment margin-small", "* 주차는 할 수 없어요" }
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
