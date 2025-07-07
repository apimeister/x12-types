pub fn now_as_tm() -> String {
    let t = chrono::Local::now().format("%H:%M:%S");
    format!("{t}")
}
