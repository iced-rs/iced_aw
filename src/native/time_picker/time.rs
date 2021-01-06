//! TODO

use std::fmt::Display;

/// TODO
#[derive(Clone, Debug)]
pub enum Time {
    /// TODO
    Hm {
        /// TODO
        hour: u32,
        /// TODO
        minute: u32,
        /// TODO
        period: Period,
    },

    /// TODO
    Hms {
        /// TODO
        hour: u32,
        /// TODO
        minute: u32,
        /// TODO
        second: u32,
        /// TODO
        period: Period,
    },
}

impl Time {
    /// TODO
    pub fn default_hm(period: Period) -> Self {
        Time::Hm {
            hour: 0,
            minute: 0,
            period: period,
        }
    }

    /// TODO
    pub fn default_hms(period: Period) -> Self {
        Time::Hms {
            hour: 0,
            minute: 0,
            second: 0,
            period: period,
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Time::Hm{hour, minute, period} => {
                write!(f, "{:02}:{:02}{}", hour, minute, period)
            },
            Time::Hms{hour, minute, second, period} => {
                write!(f, "{:02}:{:02}:{:02}{}", hour, minute, second, period)
            }
        }
    }
}

/// TODO
#[derive(Clone, Debug, PartialEq)]
pub enum Period {
    /// TODO
    H24,
    /// TODO
    Am,
    /// TODO
    Pm,
}

impl Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Period::H24 => "",
            Period::Am => " AM",
            Period::Pm => " PM",
        })
    }
}

impl From<Time> for chrono::NaiveTime {
    fn from(time: Time) -> Self {
        let (h, m, s, p) = match time {
            Time::Hm { hour, minute, period } => (hour, minute, 0, period),
            Time::Hms { hour, minute, second, period } => (hour, minute, second, period),
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