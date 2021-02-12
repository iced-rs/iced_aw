//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: time_picker*
use std::fmt::Display;

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
    /// The default time `00:00` with the given period.
    pub fn default_hm(period: Period) -> Self {
        Time::Hm {
            hour: 0,
            minute: 0,
            period,
        }
    }

    /// The default time `00:00:00` with the given period.
    pub fn default_hms(period: Period) -> Self {
        Time::Hms {
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
            Time::Hm {
                hour,
                minute,
                period,
            } => {
                write!(f, "{:02}:{:02}{}", hour, minute, period)
            }
            Time::Hms {
                hour,
                minute,
                second,
                period,
            } => {
                write!(f, "{:02}:{:02}:{:02}{}", hour, minute, second, period)
            }
        }
    }
}

/// The current period of the clock
#[derive(Clone, Copy, Debug, PartialEq)]
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
                Period::H24 => "",
                Period::Am => " AM",
                Period::Pm => " PM",
            }
        )
    }
}

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

        chrono::NaiveTime::from_hms(h, m, s)
    }
}

#[cfg(test)]
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
