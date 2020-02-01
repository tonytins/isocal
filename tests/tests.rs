use chrono::{Datelike, IsoWeek, Local, NaiveDate, Weekday};
use isodate::IsoDate;

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
