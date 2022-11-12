//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*
use std::fmt::Display;

use chrono::{Local, Timelike};

/// The time value
#[derive(Clone, Copy, Debug)]
pub enum Time {
    /// The time value containing hour, minute and period.
    Hm {
        /// The hour value.
        hour: u32,
        /// The minute value.
        minute: u32,
        /// The current period of the time.
        period: Period,
    },

    /// The time value containing hour, minute, second and period.
    Hms {
        /// The hour value.
        hour: u32,
        /// The minute value.
        minute: u32,
        /// The second value.
        second: u32,
        /// The current period of the time.
        period: Period,
    },
}

impl Time {
    /// Creates a new time (hours, minutes) from the current timestamp.
    #[must_use]
    pub fn now_hm(use_24h: bool) -> Self {
        let now = Local::now().naive_local().time();

        let (hour, period) = if use_24h {
            (now.hour(), Period::H24)
        } else {
            let (period, hour12) = now.hour12();

            (hour12, if period { Period::Pm } else { Period::Am })
        };

        Self::Hm {
            hour,
            minute: now.minute(),
            period,
        }
    }

    /// Creates a new time (hours, minutes, seconds) from the current timestamp.
    #[must_use]
    pub fn now_hms(use_24h: bool) -> Self {
        let now = Local::now().naive_local().time();

        let (hour, period) = if use_24h {
            (now.hour(), Period::H24)
        } else {
            let (period, hour12) = now.hour12();

            (hour12, if period { Period::Pm } else { Period::Am })
        };

        Self::Hms {
            hour,
            minute: now.minute(),
            second: now.second(),
            period,
        }
    }

    /// The default time `00:00` with the given period.
    #[must_use]
    pub const fn default_hm(period: Period) -> Self {
        Self::Hm {
            hour: 0,
            minute: 0,
            period,
        }
    }

    /// The default time `00:00:00` with the given period.
    #[must_use]
    pub const fn default_hms(period: Period) -> Self {
        Self::Hms {
            hour: 0,
            minute: 0,
            second: 0,
            period,
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hm {
                hour,
                minute,
                period,
            } => {
                write!(f, "{hour:02}:{minute:02}{period}")
            }
            Self::Hms {
                hour,
                minute,
                second,
                period,
            } => {
                write!(f, "{hour:02}:{minute:02}:{second:02}{period}")
            }
        }
    }
}

/// The current period of the clock
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Period {
    /// No period - using 24 hour format.
    H24,
    /// Ante meridiem: Before noon.
    Am,
    /// Post meridiem: After noon.
    Pm,
}

impl Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::H24 => "",
                Self::Am => " AM",
                Self::Pm => " PM",
            }
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<Time> for chrono::NaiveTime {
    fn from(time: Time) -> Self {
        let (h, m, s, p) = match time {
            Time::Hm {
                hour,
                minute,
                period,
            } => (hour, minute, 0, period),
            Time::Hms {
                hour,
                minute,
                second,
                period,
            } => (hour, minute, second, period),
        };

        let h = if h == 12 && p != Period::H24 { 0 } else { h };

        let h = match p {
            Period::H24 | Period::Am => h,
            Period::Pm => (h + 12) % 24,
        };

        Self::from_hms(h, m, s)
    }
}

impl From<chrono::NaiveTime> for Time {
    fn from(time: chrono::NaiveTime) -> Self {
        Self::Hms {
            hour: time.hour(),
            minute: time.minute(),
            second: time.second(),
            period: Period::H24,
        }
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use chrono::NaiveTime;

    use super::{Period, Time};

    #[test]
    fn time_to_naive() {
        let time = Time::Hms {
            hour: 8,
            minute: 52,
            second: 17,
            period: Period::H24,
        };

        let naive: NaiveTime = time.into();
        assert_eq!(naive, NaiveTime::from_hms(8, 52, 17));

        let time = Time::Hms {
            hour: 23,
            minute: 48,
            second: 39,
            period: Period::H24,
        };

        let naive: NaiveTime = time.into();
        assert_eq!(naive, NaiveTime::from_hms(23, 48, 39));

        let time = Time::Hms {
            hour: 8,
            minute: 52,
            second: 17,
            period: Period::Am,
        };

        let naive: NaiveTime = time.into();
        assert_eq!(naive, NaiveTime::from_hms(8, 52, 17));

        let time = Time::Hms {
            hour: 8,
            minute: 52,
            second: 17,
            period: Period::Pm,
        };

        let naive: NaiveTime = time.into();
        assert_eq!(naive, NaiveTime::from_hms(20, 52, 17));

        let time = Time::Hms {
            hour: 12,
            minute: 52,
            second: 17,
            period: Period::Am,
        };

        let naive: NaiveTime = time.into();
        assert_eq!(naive, NaiveTime::from_hms(0, 52, 17));

        let time = Time::Hms {
            hour: 12,
            minute: 52,
            second: 17,
            period: Period::Pm,
        };

        let naive: NaiveTime = time.into();
        assert_eq!(naive, NaiveTime::from_hms(12, 52, 17));

        let time = Time::Hm {
            hour: 17,
            minute: 52,
            period: Period::H24,
        };

        let naive: NaiveTime = time.into();
        assert_eq!(naive, NaiveTime::from_hms(17, 52, 0));
    }
}
