/*
 * This project is licensed under the MIT license.
 * See the LICENSE file in the project root for more information.
 */
/// IsoDate is an extension to Chrono's IsoWeek (ISO 8601) that provides additional
/// formatting and functions, such as the "W" prefix to weeks and the ISO ordinal.
use chrono::{DateTime, Datelike, FixedOffset, IsoWeek, Local, NaiveDate, Utc};


pub trait IsoDate {
    fn week_fancy(self) -> String;
    fn week0_fancy(self) -> String;
    fn date(self) -> String;
    fn date0(self) -> String;
}

pub trait IsoCal {
    fn iso_ordinal(self) -> u32;
    fn iso_ordinal0(self) -> u32;
}

impl IsoCal for NaiveDate {
    /// Returns the day of the day year.
    ///
    /// The return value ranges from 1 to 371. (The last day of year differs by years.)
    fn iso_ordinal(self) -> u32 {
        if self.year() % 4 == 0 {
            self.ordinal() + 5
        } else {
            self.ordinal() - 1
        }
    }

    /// Returns the day of the day year.
    ///
    /// The return value ranges from 0 to 371. (The last day of year differs by years.)
    fn iso_ordinal0(self) -> u32 {
        if self.year() % 4 == 0 {
            self.ordinal0() + 6
        } else {
            self.ordinal0()
        }
    }
}

impl IsoCal for DateTime<Utc> {
    /// Returns the day of the day year.
    ///
    /// The return value ranges from 1 to 371. (The last day of year differs by years.)
    fn iso_ordinal(self) -> u32 {
        if self.year() % 4 == 0 {
            self.ordinal() + 5
        } else {
            self.ordinal() - 1
        }
    }

    /// Returns the day of the day year.
    ///
    /// The return value ranges from 0 to 371. (The last day of year differs by years.)
    fn iso_ordinal0(self) -> u32 {
        if self.year() % 4 == 0 {
            self.ordinal0() + 6
        } else {
            self.ordinal0()
        }
    }
}

impl IsoCal for DateTime<Local> {
    /// Returns the day of the day year.
    ///
    /// The return value ranges from 1 to 371. (The last day of year differs by years.)
    fn iso_ordinal(self) -> u32 {
        if self.year() % 4 == 0 {
            self.ordinal() + 5
        } else {
            self.ordinal() - 1
        }
    }

    /// Returns the day of the day year.
    ///
    /// The return value ranges from 0 to 371. (The last day of year differs by years.)
    fn iso_ordinal0(self) -> u32 {
        if self.year() % 4 == 0 {
            self.ordinal0() + 6
        } else {
            self.ordinal0()
        }
    }
}

impl IsoCal for DateTime<FixedOffset> {
    /// Returns the day of the day year.
    ///
    /// The return value ranges from 1 to 371. (The last day of year differs by years.)
    fn iso_ordinal(self) -> u32 {
        if self.year() % 4 == 0 {
            self.ordinal() + 5
        } else {
            self.ordinal() - 1
        }
    }

    /// Returns the day of the day year.
    ///
    /// The return value ranges from 0 to 371. (The last day of year differs by years.)
    fn iso_ordinal0(self) -> u32 {
        if self.year() % 4 == 0 {
            self.ordinal0() + 6
        } else {
            self.ordinal0()
        }
    }
}

impl IsoDate for IsoWeek {
    /// Returns the ISO week with a W prefix.
    /// The return value ranges from 0 to 53. (The last week of year differs by years.)
    /// ```
    /// extern crate isocal; // Rust 2015
    /// use isocal::IsoDate;
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
    /// extern crate isocal; // Rust 2015
    /// use isocal::IsoDate;
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
