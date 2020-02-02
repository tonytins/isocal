use chrono::{Datelike, NaiveDate, Weekday};
use isocal::{IsoCal, IsoDate};

#[test]
fn week_test() {
    let nd = NaiveDate::from_isoywd(2015, 1, Weekday::Mon);
    let actual = nd.iso_week();
    assert_eq!(actual.week_fancy(), "W01");
}

#[test]
fn week0_test() {
    let nd = NaiveDate::from_isoywd(2015, 1, Weekday::Mon);
    let actual = nd.iso_week();
    assert_eq!(actual.week0_fancy(), "W00");
}

#[test]
fn date_test() {
    let nd = NaiveDate::from_isoywd(2015, 1, Weekday::Mon);
    let actual = nd.iso_week();
    assert_eq!(actual.date(), "2015-W01");
}

#[test]
fn date0_test() {
    let nd = NaiveDate::from_isoywd(2015, 1, Weekday::Mon);
    let actual = nd.iso_week();
    assert_eq!(actual.date0(), "2015-W00");
}

#[test]
fn ordinal_test() {
    let dt = NaiveDate::from_ymd(2015, 12, 31);
    assert_eq!(dt.iso_ordinal(), 364);
    assert_eq!(dt.iso_ordinal0(), 364);
}

#[test]
fn ordinal_leap_test() {
    let dt = NaiveDate::from_ymd(2020, 12, 31);
    assert_eq!(dt.iso_ordinal(), 371);
    assert_eq!(dt.iso_ordinal0(), 371);
}
