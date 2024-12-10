use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    ldap: String,
}

pub fn Card(props: CardProps) -> Element {
    rsx! {
        div { class: "container-vertical", id: "snow-container",
            script { src: asset!("./assets/snowflake.js") }
            img {
                id: "snowflake",
                alt: "Snowflake",
                class: "margin-large",
                src: asset!("./assets/snowflake.svg"),
                style: "width: 20vh; height: 20vh"
            }
            p { class: "font-title margin-large", "송년회 초대장" }
            p { class: "font-body margin-small",
                "2024년 광고서버플랫폼개발 송년회에 초대합니다 {props.ldap}"
            }

            div { class: "horizontal-line", aria_hidden: true }
            p { class: "font-subtitle margin-medium", "날짜 및 위치" }
            p { class: "font-body margin-small", "2024년 12월 18일 18:00" }
            p { class: "font-body margin-small", "경기도 성남시 분당구 서현로357번길 6" }
            p { class: "font-body margin-small",
                "데일리 파티룸 "
                a { href: "https://place.map.kakao.com/1841089335", "[Link]" }
            }
            p { class: "font-comment margin-small", "* 주차는 할 수 없어요" }
            Map {}

            div { class: "horizontal-line", aria_hidden: true }
            p { class: "font-subtitle margin-medium", "스케줄" }
            Schedule { step: 0 }
        }
    }
}

fn Map() -> Element {
    rsx! {
        div {
            id: "map",
            class: "margin-large",
            style: "border-radius: 10px; width: 30vh; height: 30vh;",
            script { src: asset!("./assets/map.js") }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
struct ScheduleProps {
    step: i32,
}

fn Schedule(props: ScheduleProps) -> Element {
    rsx! {
        div { class: "container-vertical margin-large",
            div { class: "container-horizontal",
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: end;",
                    "18:00"
                }
                div { class: "schedule-cell" }
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: start;",
                    "파티룸 예약 시간"
                }
            }
            div { class: "container-horizontal",
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: end;",
                    "18:00"
                }
                div { class: "schedule-cell" }
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: start;",
                    "파티룸 예약 시간"
                }
            }
            div { class: "container-horizontal",
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: end;",
                    "18:00"
                }
                div { class: "schedule-cell" }
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: start;",
                    "파티룸 예약 시간"
                }
            }
            div { class: "container-horizontal",
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: end;",
                    "18:00"
                }
                div { class: "schedule-cell" }
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: start;",
                    "파티룸 예약 시간"
                }
            }
            div { class: "container-horizontal",
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: end;",
                    "18:00"
                }
                div { class: "schedule-cell" }
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: start;",
                    "파티룸 예약 시간"
                }
            }
            div { class: "container-horizontal",
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: end;",
                    "18:00"
                }
                div { class: "schedule-cell" }
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: start;",
                    "파티룸 예약 시간"
                }
            }
            div { class: "container-horizontal",
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: end;",
                    "18:00"
                }
                div { class: "schedule-cell" }
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: start;",
                    "파티룸 예약 시간"
                }
            }
            div { class: "container-horizontal",
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: end;",
                    "18:00"
                }
                div { class: "schedule-cell" }
                p {
                    class: "font-body schedule-cell",
                    style: "text-align: start;",
                    "파티룸 예약 시간"
                }
            }
            div {
                class: "container-vertical",
                style: "position: absolute; width: fit-content; height: fit-content;",
                div {
                    class: "vertical-line-passed",
                    style: "height: {35+75*props.step}px;"
                }
                div { class: "vertical-line-circle" }
                div {
                    class: "vertical-line-remaining",
                    style: "height: {565-75*props.step}px;"
                }
            }
        }
    }
}
