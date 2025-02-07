//! Helper functions for calculating dates

use chrono::{Datelike, Duration, Local, NaiveDate};
use std::fmt::Display;
use std::sync::LazyLock;

/// The date value
#[derive(Clone, Copy, Debug)]
pub struct Date {
    /// The year value of the date.
    pub year: i32,
    /// The month value of the date (1 - 12).
    pub month: u32,
    /// The day value of the date (1 - 31).
    pub day: u32,
}

impl Default for Date {
    fn default() -> Self {
        Self {
            year: 2024,
            month: 1,
            day: 1,
        }
    }
}

impl Date {
    /// Creates a new date from the current timestamp.
    #[must_use]
    pub fn today() -> Self {
        let today = Local::now().naive_local().date();
        today.into()
    }

    /// Creates a new date.
    #[must_use]
    pub const fn from_ymd(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl From<Date> for NaiveDate {
    fn from(date: Date) -> Self {
        Self::from_ymd_opt(date.year, date.month, date.day)
            .expect("Year, Month or Day doesnt Exist")
    }
}

impl From<NaiveDate> for Date {
    fn from(date: NaiveDate) -> Self {
        Self::from_ymd(date.year(), date.month(), date.day())
    }
}

/// # Panics
/// Creates a date with the previous month based on the given date.
/// panics if year, month or day doesnt exist.
#[must_use]
pub fn pred_month(date: NaiveDate) -> NaiveDate {
    let (year, month) = if date.month() == 1 {
        (date.year() - 1, 12)
    } else {
        (date.year(), date.month() - 1)
    };

    let day = date.day().min(num_days_of_month(year, month));

    NaiveDate::from_ymd_opt(year, month, day).expect("Year, Month or Day doesnt Exist")
}

/// # Panics
/// Creates a date with the next month based on given date.
/// panics if year, month or day doesnt exist.
#[must_use]
pub fn succ_month(date: NaiveDate) -> NaiveDate {
    let (year, month) = if date.month() == 12 {
        (date.year() + 1, 1)
    } else {
        (date.year(), date.month() + 1)
    };

    let day = date.day().min(num_days_of_month(year, month));

    NaiveDate::from_ymd_opt(year, month, day).expect("Year, Month or Day doesnt Exist")
}

/// # Panics
/// Creates a date with the previous year based on the given date.
// panics if year, month or day doesnt exist.
#[must_use]
pub fn pred_year(date: NaiveDate) -> NaiveDate {
    let year = date.year() - 1;
    let day = date.day().min(num_days_of_month(year, date.month()));

    NaiveDate::from_ymd_opt(year, date.month(), day).expect("Year, Month or Day doesnt Exist")
}

/// # Panics
/// Creates a date with the next year based on the given date.
// panics if year, month or day doesnt exist.
#[must_use]
pub fn succ_year(date: NaiveDate) -> NaiveDate {
    let year = date.year() + 1;
    let day = date.day().min(num_days_of_month(year, date.month()));

    NaiveDate::from_ymd_opt(year, date.month(), day).expect("Year, Month or Day doesnt Exist")
}

/// Calculates a date with the previous week based on the given date.
#[must_use]
pub fn pred_week(date: NaiveDate) -> NaiveDate {
    date - Duration::days(7)
}

/// Calculates a date with the next week based on the given date.
#[must_use]
pub fn succ_week(date: NaiveDate) -> NaiveDate {
    date + Duration::days(7)
}

/// Calculates a date with the previous day based on the given date.
#[must_use]
pub fn pred_day(date: NaiveDate) -> NaiveDate {
    date - Duration::days(1)
}

/// Calculates a date with the next day based on the given date.
#[must_use]
pub fn succ_day(date: NaiveDate) -> NaiveDate {
    date + Duration::days(1)
}

/// Specifies if the calculated day lays in the previous, same or next month of
/// the date.
#[derive(Debug, PartialEq, Eq)]
pub enum IsInMonth {
    /// The day lays in the previous month.
    Previous,

    /// The day lays in the same month.
    Same,

    /// The day lays in the next month.
    Next,
}

/// # Panics
/// Calculates the day number at the given position in the calendar table based
/// on the given year and month.
/// panics if year, month or day does not exist.
#[must_use]
pub fn position_to_day(x: usize, y: usize, year: i32, month: u32) -> (usize, IsInMonth) {
    let (x, y) = (x as isize, y as isize);
    let first_day =
        NaiveDate::from_ymd_opt(year, month, 1).expect("Year, Month or Day doesnt Exist");
    let day_of_week = first_day.weekday().num_days_from_monday() as isize;
    let day_of_week = if day_of_week == 0 { 7 } else { day_of_week };

    let day = (x + 7 * y) + 1 - day_of_week;

    if day < 1 {
        let last_month = first_day.pred_opt().unwrap_or_default();
        (
            (num_days_of_month(last_month.year(), last_month.month()) as isize + day) as usize,
            IsInMonth::Previous,
        )
    } else if day > num_days_of_month(year, month) as isize {
        (
            (day - num_days_of_month(year, month) as isize) as usize,
            IsInMonth::Next,
        )
    } else {
        (day as usize, IsInMonth::Same)
    }
}

/// Checks if the given year is a leap year.
const fn is_leap_year(year: i32) -> bool {
    let mod4 = year % 4 == 0;
    let mod100 = year % 100 == 0;
    let mod400 = year % 400 == 0;

    mod4 && (!mod100 || mod400)
}

/// Gets the number of days the given month in the year has.
const fn num_days_of_month(year: i32, month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 31,
    }
}

/// Gets the string representation of the date of the given month and date.
#[must_use]
pub fn date_as_string(date: NaiveDate) -> String {
    date.format("%Y %B").to_string()
}

/// Gets the length of the longest month name.
pub static MAX_MONTH_STR_LEN: LazyLock<usize> = LazyLock::new(|| {
    let months = [
        NaiveDate::from_ymd_opt(0, 1, 1).expect("Year, Month or Day doesnt Exist"),
        NaiveDate::from_ymd_opt(0, 2, 1).expect("Year, Month or Day doesnt Exist"),
        NaiveDate::from_ymd_opt(0, 3, 1).expect("Year, Month or Day doesnt Exist"),
        NaiveDate::from_ymd_opt(0, 4, 1).expect("Year, Month or Day doesnt Exist"),
        NaiveDate::from_ymd_opt(0, 5, 1).expect("Year, Month or Day doesnt Exist"),
        NaiveDate::from_ymd_opt(0, 6, 1).expect("Year, Month or Day doesnt Exist"),
        NaiveDate::from_ymd_opt(0, 7, 1).expect("Year, Month or Day doesnt Exist"),
        NaiveDate::from_ymd_opt(0, 8, 1).expect("Year, Month or Day doesnt Exist"),
        NaiveDate::from_ymd_opt(0, 9, 1).expect("Year, Month or Day doesnt Exist"),
        NaiveDate::from_ymd_opt(0, 10, 1).expect("Year, Month or Day doesnt Exist"),
        NaiveDate::from_ymd_opt(0, 11, 1).expect("Year, Month or Day doesnt Exist"),
        NaiveDate::from_ymd_opt(0, 12, 1).expect("Year, Month or Day doesnt Exist"),
    ];

    let max = months
        .iter()
        .map(|m| {
            let d = date_as_string(*m);
            let (_, s) = d.split_once(' ').expect("Date must contain space");
            s.len()
        })
        .max()
        .expect("There should be a maximum element");

    max
});

/// Gets the labels of the weekdays containing the first two characters of
/// the weekdays.
pub static WEEKDAY_LABELS: LazyLock<Vec<String>> = LazyLock::new(|| {
    let days = [
        // Monday
        NaiveDate::from_ymd_opt(2020, 6, 1).expect("Year, Month or Day doesnt Exist"),
        // Tuesday
        NaiveDate::from_ymd_opt(2020, 6, 2).expect("Year, Month or Day doesnt Exist"),
        // Wednesday
        NaiveDate::from_ymd_opt(2020, 6, 3).expect("Year, Month or Day doesnt Exist"),
        // Thursday
        NaiveDate::from_ymd_opt(2020, 6, 4).expect("Year, Month or Day doesnt Exist"),
        // Friday
        NaiveDate::from_ymd_opt(2020, 6, 5).expect("Year, Month or Day doesnt Exist"),
        // Saturday
        NaiveDate::from_ymd_opt(2020, 6, 6).expect("Year, Month or Day doesnt Exist"),
        // Sunday
        NaiveDate::from_ymd_opt(2020, 6, 7).expect("Year, Month or Day doesnt Exist"),
    ];

    days.iter()
        .map(|d| d.format("%a").to_string())
        .map(|s| s[0..2].to_owned())
        .collect()
});

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::{
        is_leap_year, num_days_of_month, position_to_day, pred_month, pred_year, succ_month,
        succ_year, IsInMonth,
    };

    #[test]
    fn pred_month_test() {
        let date = NaiveDate::from_ymd_opt(2020, 5, 6).expect("Year, Month or Day doesnt Exist");
        let result = pred_month(date);
        let expected =
            NaiveDate::from_ymd_opt(2020, 4, 6).expect("Year, Month or Day doesnt Exist");
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd_opt(2020, 1, 24).expect("Year, Month or Day doesnt Exist");
        let result = pred_month(date);
        let expected =
            NaiveDate::from_ymd_opt(2019, 12, 24).expect("Year, Month or Day doesnt Exist");
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd_opt(2020, 3, 31).expect("Year, Month or Day doesnt Exist");
        let result = pred_month(date);
        let expected =
            NaiveDate::from_ymd_opt(2020, 2, 29).expect("Year, Month or Day doesnt Exist");
        assert_eq!(result, expected);
    }

    #[test]
    fn succ_month_test() {
        let date = NaiveDate::from_ymd_opt(2020, 5, 6).expect("Year, Month or Day doesnt Exist");
        let result = succ_month(date);
        let expected =
            NaiveDate::from_ymd_opt(2020, 6, 6).expect("Year, Month or Day doesnt Exist");
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd_opt(2019, 12, 24).expect("Year, Month or Day doesnt Exist");
        let result = succ_month(date);
        let expected =
            NaiveDate::from_ymd_opt(2020, 1, 24).expect("Year, Month or Day doesnt Exist");
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd_opt(2020, 1, 31).expect("Year, Month or Day doesnt Exist");
        let result = succ_month(date);
        let expected =
            NaiveDate::from_ymd_opt(2020, 2, 29).expect("Year, Month or Day doesnt Exist");
        assert_eq!(result, expected);
    }

    #[test]
    fn pred_year_test() {
        let date = NaiveDate::from_ymd_opt(2020, 5, 6).expect("Year, Month or Day doesnt Exist");
        let result = pred_year(date);
        let expected =
            NaiveDate::from_ymd_opt(2019, 5, 6).expect("Year, Month or Day doesnt Exist");
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd_opt(2020, 2, 29).expect("Year, Month or Day doesnt Exist");
        let result = pred_year(date);
        let expected =
            NaiveDate::from_ymd_opt(2019, 2, 28).expect("Year, Month or Day doesnt Exist");
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd_opt(2021, 2, 28).expect("Year, Month or Day doesnt Exist");
        let result = pred_year(date);
        let expected =
            NaiveDate::from_ymd_opt(2020, 2, 28).expect("Year, Month or Day doesnt Exist");
        assert_eq!(result, expected);
    }

    #[test]
    fn succ_year_test() {
        let date = NaiveDate::from_ymd_opt(2020, 5, 6).expect("Year, Month or Day doesnt Exist");
        let result = succ_year(date);
        let expected =
            NaiveDate::from_ymd_opt(2021, 5, 6).expect("Year, Month or Day doesnt Exist");
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd_opt(2020, 2, 29).expect("Year, Month or Day doesnt Exist");
        let result = succ_year(date);
        let expected =
            NaiveDate::from_ymd_opt(2021, 2, 28).expect("Year, Month or Day doesnt Exist");
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd_opt(2019, 2, 28).expect("Year, Month or Day doesnt Exist");
        let result = succ_year(date);
        let expected =
            NaiveDate::from_ymd_opt(2020, 2, 28).expect("Year, Month or Day doesnt Exist");
        assert_eq!(result, expected);
    }

    #[allow(clippy::shadow_unrelated)]
    #[test]
    fn position_to_day_test() {
        let (day, is_in_month) = position_to_day(0, 0, 2020, 12);
        assert_eq!(day, 30);
        assert_eq!(is_in_month, IsInMonth::Previous);

        let (day, is_in_month) = position_to_day(1, 0, 2020, 12);
        assert_eq!(day, 1);
        assert_eq!(is_in_month, IsInMonth::Same);

        let (day, is_in_month) = position_to_day(3, 4, 2020, 12);
        assert_eq!(day, 31);
        assert_eq!(is_in_month, IsInMonth::Same);

        let (day, is_in_month) = position_to_day(6, 5, 2020, 12);
        assert_eq!(day, 10);
        assert_eq!(is_in_month, IsInMonth::Next);

        let (day, is_in_month) = position_to_day(0, 0, 2020, 11);
        assert_eq!(day, 26);
        assert_eq!(is_in_month, IsInMonth::Previous);

        let (day, is_in_month) = position_to_day(6, 0, 2020, 11);
        assert_eq!(day, 1);
        assert_eq!(is_in_month, IsInMonth::Same);

        let (day, is_in_month) = position_to_day(0, 5, 2020, 11);
        assert_eq!(day, 30);
        assert_eq!(is_in_month, IsInMonth::Same);

        let (day, is_in_month) = position_to_day(6, 5, 2020, 11);
        assert_eq!(day, 6);
        assert_eq!(is_in_month, IsInMonth::Next);

        let (day, is_in_month) = position_to_day(0, 0, 2021, 2);
        assert_eq!(day, 25);
        assert_eq!(is_in_month, IsInMonth::Previous);

        let (day, is_in_month) = position_to_day(0, 1, 2021, 2);
        assert_eq!(day, 1);
        assert_eq!(is_in_month, IsInMonth::Same);

        let (day, is_in_month) = position_to_day(6, 4, 2021, 2);
        assert_eq!(day, 28);
        assert_eq!(is_in_month, IsInMonth::Same);

        let (day, is_in_month) = position_to_day(0, 5, 2021, 2);
        assert_eq!(day, 1);
        assert_eq!(is_in_month, IsInMonth::Next);
    }

    #[test]
    fn is_leap_year_test() {
        assert!(is_leap_year(2020));
        assert!(!is_leap_year(2019));
        assert!(!is_leap_year(2021));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1000));
    }

    #[test]
    fn num_days_of_month_test() {
        assert_eq!(num_days_of_month(2020, 1), 31);
        assert_eq!(num_days_of_month(2020, 2), 29);
        assert_eq!(num_days_of_month(2019, 2), 28);
        assert_eq!(num_days_of_month(2020, 3), 31);
        assert_eq!(num_days_of_month(2020, 4), 30);
        assert_eq!(num_days_of_month(2020, 5), 31);
        assert_eq!(num_days_of_month(2020, 6), 30);
        assert_eq!(num_days_of_month(2020, 7), 31);
        assert_eq!(num_days_of_month(2020, 8), 31);
        assert_eq!(num_days_of_month(2020, 9), 30);
        assert_eq!(num_days_of_month(2020, 10), 31);
        assert_eq!(num_days_of_month(2020, 11), 30);
        assert_eq!(num_days_of_month(2020, 12), 31);
    }
}
