use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::{Asia::Seoul, Tz};
use dioxus::prelude::*;
use std::cmp::min;

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
            p { class: "font-body margin-small", "반가워요 {props.ldap}!" }
            p { class: "font-body margin-small", "2024년 ASPD 송년회에 초대합니다" }

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
            Schedule { now: Seoul.from_utc_datetime(&Utc::now().naive_local()) }
        }
    }
}

fn Map() -> Element {
    rsx! {
        div {
            id: "map",
            class: "margin-large",
            style: "border-radius: 10px; width: 300px; height: 300px;",
            script { src: asset!("./assets/map.js") }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
struct ScheduleProps {
    now: DateTime<Tz>,
}

impl ScheduleProps {
    fn step(&self) -> i32 {
        let mut step = 0;
        for (time, _title) in self.times() {
            if time < self.now {
                step += 1;
            }
        }
        return min(step, (self.times().len() - 1) as i32);
    }

    fn times(&self) -> Vec<(DateTime<Tz>, &str)> {
        return vec![
            (
                Seoul.with_ymd_and_hms(2024, 12, 18, 18, 0, 0).unwrap(),
                "파티룸 예약 시간",
            ),
            (
                Seoul.with_ymd_and_hms(2024, 12, 18, 18, 0, 0).unwrap(),
                "파티룸 예약 시간",
            ),
            (
                Seoul.with_ymd_and_hms(2024, 12, 18, 18, 0, 0).unwrap(),
                "파티룸 예약 시간",
            ),
            (
                Seoul.with_ymd_and_hms(2024, 12, 18, 18, 0, 0).unwrap(),
                "파티룸 예약 시간",
            ),
            (
                Seoul.with_ymd_and_hms(2024, 12, 18, 18, 0, 0).unwrap(),
                "파티룸 예약 시간",
            ),
            (
                Seoul.with_ymd_and_hms(2024, 12, 18, 18, 0, 0).unwrap(),
                "파티룸 예약 시간",
            ),
            (
                Seoul.with_ymd_and_hms(2024, 12, 18, 18, 0, 0).unwrap(),
                "파티룸 예약 시간",
            ),
            (
                Seoul.with_ymd_and_hms(2024, 12, 18, 18, 0, 0).unwrap(),
                "파티룸 예약 시간",
            ),
        ];
    }
}

fn Schedule(props: ScheduleProps) -> Element {
    const HOUR_FORMAT: &str = "%H:%M";
    rsx! {
        div { class: "container-vertical margin-large",
            for (foo , title) in props.times().iter() {
                div { class: "container-horizontal",
                    p {
                        class: "font-body schedule-time",
                        style: "text-align: end;",
                        "{foo.format(HOUR_FORMAT).to_string()}"
                    }
                    div { style: "width: 80px;" }
                    p {
                        class: "font-body schedule-title",
                        style: "text-align: start;",
                        "{title}"
                    }
                }
            }
            div {
                class: "container-vertical",
                style: "position: absolute; width: fit-content; height: fit-content; margin-right: 60px;",
                div {
                    class: "vertical-line-passed",
                    style: "height: {40+66*props.step()}px;"
                }
                div { class: "vertical-line-circle" }
                div {
                    class: "vertical-line-remaining",
                    style: "height: {510-66*props.step()}px;"
                }
            }
        }
    }
}
