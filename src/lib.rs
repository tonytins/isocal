/*
 * This project is licensed under the MIT license.
 * See the LICENSE file in the project root for more information.
 */
/// IsoWeekDate extends Chrono's IsoWeek by providing proper formatting
/// and the complete date.
use chrono::IsoWeek;

pub trait IsoDate {
    fn week_fancy(self) -> String;
    fn week0_fancy(self) -> String;
    fn date(self) -> String;
    fn date0(self) -> String;
}

impl IsoDate for IsoWeek {
    
    /// Returns the ISO week with a W prefix.
    /// The return value ranges from 0 to 53. (The last week of year differs by years.)
    /// ```
    /// extern crate isodate; // Rust 2015
    /// use isodate::IsoDate;
    /// use chrono::{Local, IsoWeek, Datelike};
    ///
    /// fn main() {
    ///     let now = Local::now();
    ///     let isow = now.iso_week();
    ///
    ///     println!("{}", isow.week_fancy());
    /// }
    /// ```
    fn week_fancy(self) -> String {
        format!("W{:02}", self.week())
    }

    /// Returns the ISO week with a W prefix.
    /// The return value ranges from 0 to 52. (The last week of year differs by years.)
    fn week0_fancy(self) -> String {
        format!("W{:02}", self.week0())
    }

    /// Returns the ISO year and week.
    /// The week ranges from 0 to 53. (The last week of year differs by years.)
    /// ```
    /// extern crate isodate; // Rust 2015
    /// use isodate::IsoDate;
    /// use chrono::{Local, IsoWeek, Datelike};
    ///
    /// fn main() {
    ///     let now = Local::now();
    ///     let isow = now.iso_week();
    ///
    ///     println!("{}", isow.date());
    /// }
    /// ```
    fn date(self) -> String {
        format!("{}-{}", self.year(), self.week_fancy())
    }

    /// Returns the ISO year and week.
    /// The week ranges from 0 to 52. (The last week of year differs by years.)
    fn date0(self) -> String {
        format!("{}-{}", self.year(), self.week0_fancy())
    }
}
