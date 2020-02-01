use isodate::IsoDate;
use chrono::{Local, Utc, Datelike};

pub fn main() {
    let local = Local::now();
    let utc = Utc::now();
    let utc_isow = utc.iso_week();
    let local_isow = local.iso_week();

    println!("Local: {}-{}, {}", local_isow.date(), local.day(), local.time());
    println!("UTC: {}-{}, {}", utc_isow.date(), utc.day(), utc.time());
}