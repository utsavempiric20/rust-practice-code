use chrono::{Local, Utc};

fn main() {
    let local_time = Local::now();
    println!("Local time : {}", local_time);

    let utc_time = Utc::now();
    println!("utcTime : {}", utc_time);
}
