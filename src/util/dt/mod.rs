pub fn now_as_dt() -> String {
    let t = chrono::Local::now().format("%Y-%m-%d");
    format!("{t}")
}

// pub fn datetime_as_dt<Tz: TimeZone>(dt: DateTime<Tz>) -> String {
//     let t = dt.format("%Y-%m-%d");
//     format!("{}",t)
// }
