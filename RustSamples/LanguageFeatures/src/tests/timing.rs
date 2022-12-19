use rand::Rng;
use std::time::{Instant, SystemTime};
use chrono::prelude::{DateTime, Utc};

#[test]
fn delta() {
    let start = Instant::now();
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("{} {}", random_num, start.elapsed().as_nanos());
}

fn to_iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%+"))
}

fn from_iso8601(s: String) -> std::time::SystemTime {
    // todo
    return SystemTime::now();
}

#[test]
fn datetime() {
    println!("{}", to_iso8601(&SystemTime::now()));
}

