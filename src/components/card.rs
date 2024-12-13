use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::{Asia::Seoul, Tz};
use dioxus::prelude::*;
use std::{cmp::min, time::Duration};

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    ldap: String,
}

async fn check_team(ldap: &str) {
    let team_one = option_env!("TEAM_ONE")
        .or(Some("foo.one,bar.one,baz.one"))
        .expect("$TEAM_ONE not set")
        .split(",");
    let team_two = option_env!("TEAM_TWO")
        .or(Some("foo.two,bar.two,baz.two"))
        .expect("$TEAM_TWO not set")
        .split(",");
    let team_three = option_env!("TEAM_THREE")
        .or(Some("foo.three,bar.three,baz.three"))
        .expect("$TEAM_THREE not set")
        .split(",");
    let team_four = option_env!("TEAM_FOUR")
        .or(Some("foo.four,bar.four,baz.four"))
        .expect("$TEAM_FOUR not set")
        .split(",");

    let mut team_number: usize = 0;
    for (idx, team) in vec![team_one, team_two, team_three, team_four]
        .iter_mut()
        .enumerate()
    {
        match team.find(|&m| ldap == m) {
            Some(_) => {
                team_number = idx + 1;
                break;
            }
            None => 0,
        };
    }

    document::eval(
        r#"
            let hint = document.getElementById("hint");
            let recv = await dioxus.recv();
            switch (recv) {
            case 1:
                hint.classList.add("team_one");
                break;
            case 2:
                hint.classList.add("team_two");
                break;
            case 3:
                hint.classList.add("team_three");
                break;
            case 4:
                hint.classList.add("team_four");
                break;
            default:
                hint.classList.add("team_none");
            }
        "#,
    )
    .send(team_number)
    .unwrap();
}

pub fn Card(props: CardProps) -> Element {
    let ldap = use_signal(|| props.ldap.clone());
    spawn(async move {
        check_team(&ldap()).await;
    });

    let mut now = use_signal(|| Seoul.from_utc_datetime(&Utc::now().naive_local()));
    use_coroutine::<Coroutine<()>, _, _>(move |_rx| async move {
        loop {
            async_std::task::sleep(Duration::from_secs(5)).await;
            now.set(Seoul.from_utc_datetime(&Utc::now().naive_local()));
        }
    });

    rsx! {
        div { class: "container-vertical", id: "snow-container",
            script { src: asset!("./assets/snowflake.js") }
            img {
                id: "snowflake",
                alt: "Snowflake",
                class: "margin-large",
                src: asset!("./assets/snowflake.svg"),
                style: "width: 20vh; height: 20vh",
            }
            p { class: "font-title margin-large", "송년회 초대장" }
            div { id: "hint", ontouchstart: |_| (),
                p { class: "font-body margin-small", ontouchstart: |_| (),
                    "반가워요 {props.ldap}!"
                }
            }

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
            Schedule { now: *now.read_unchecked() }
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
                "시작",
            ),
            (
                Seoul.with_ymd_and_hms(2024, 12, 18, 18, 30, 0).unwrap(),
                "아이스브레이킹",
            ),
            (
                Seoul.with_ymd_and_hms(2024, 12, 18, 19, 0, 0).unwrap(),
                "선물 쟁탈전",
            ),
            (
                Seoul.with_ymd_and_hms(2024, 12, 18, 20, 0, 0).unwrap(),
                "배 채우기",
            ),
            (
                Seoul.with_ymd_and_hms(2024, 12, 18, 20, 30, 0).unwrap(),
                "연말 결산",
            ),
            (
                Seoul.with_ymd_and_hms(2024, 12, 18, 21, 10, 0).unwrap(),
                "끝 인사",
            ),
            (
                Seoul.with_ymd_and_hms(2024, 12, 18, 21, 30, 0).unwrap(),
                "종료",
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
                    style: "height: {50+65*props.step()}px;",
                }
                div { class: "vertical-line-circle" }
                div {
                    class: "vertical-line-remaining",
                    style: "height: {450-65*props.step()}px;",
                }
            }
        }
    }
}
