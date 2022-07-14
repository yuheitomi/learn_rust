use chrono::prelude::*;

fn main() {
    time("2022-10-11");
}

fn time(start: &str) {
    let start_time = "9:30";
    let start = format!("{} {}", start, start_time);

    let naive_time = NaiveDateTime::parse_from_str(&start, "%Y-%m-%d %H:%M").unwrap();
    println!("Naive time: {}", naive_time);

    let local_time = Local.from_local_datetime(&naive_time).unwrap();
    println!("Local time: {}", local_time);
    print_time(&local_time);

    let jst_time = local_time.with_timezone(&FixedOffset::east(-4 * 3600));
    println!("JST time: {}", jst_time);
    print_time(&jst_time.into());
}

fn print_time(time: &DateTime<Local>) {
    println!("Time: {}", time);
}