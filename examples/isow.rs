use chrono::{Datelike, Local};
use isocal::{IsoCal, IsoDate};

pub fn main() {
    let dt = Local::now();
    let isow = dt.iso_week();

    println!("ISO Date: {}-{}T{}", isow.date(), dt.day(), dt.time());
    println!(
        "It's day {} of the {} ISO calendar year.",
        dt.iso_ordinal(),
        isow.year()
    );
}
