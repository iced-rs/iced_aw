//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*

use chrono::{Local, Timelike};
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

impl Default for Time {
    fn default() -> Self {
        Self::Hm {
            hour: 1,
            minute: 0,
            period: Period::Am,
        }
    }
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

        Self::from_hms_opt(h, m, s).expect("Time Conversion failed. H, M, or S was too large.")
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
        assert_eq!(
            naive,
            NaiveTime::from_hms_opt(8, 52, 17).expect("Time Conversion failed")
        );

        let time = Time::Hms {
            hour: 23,
            minute: 48,
            second: 39,
            period: Period::H24,
        };

        let naive: NaiveTime = time.into();
        assert_eq!(
            naive,
            NaiveTime::from_hms_opt(23, 48, 39).expect("Time Conversion failed")
        );

        let time = Time::Hms {
            hour: 8,
            minute: 52,
            second: 17,
            period: Period::Am,
        };

        let naive: NaiveTime = time.into();
        assert_eq!(
            naive,
            NaiveTime::from_hms_opt(8, 52, 17).expect("Time Conversion failed")
        );

        let time = Time::Hms {
            hour: 8,
            minute: 52,
            second: 17,
            period: Period::Pm,
        };

        let naive: NaiveTime = time.into();
        assert_eq!(
            naive,
            NaiveTime::from_hms_opt(20, 52, 17).expect("Time Conversion failed")
        );

        let time = Time::Hms {
            hour: 12,
            minute: 52,
            second: 17,
            period: Period::Am,
        };

        let naive: NaiveTime = time.into();
        assert_eq!(
            naive,
            NaiveTime::from_hms_opt(0, 52, 17).expect("Time Conversion failed")
        );

        let time = Time::Hms {
            hour: 12,
            minute: 52,
            second: 17,
            period: Period::Pm,
        };

        let naive: NaiveTime = time.into();
        assert_eq!(
            naive,
            NaiveTime::from_hms_opt(12, 52, 17).expect("Time Conversion failed")
        );

        let time = Time::Hm {
            hour: 17,
            minute: 52,
            period: Period::H24,
        };

        let naive: NaiveTime = time.into();
        assert_eq!(
            naive,
            NaiveTime::from_hms_opt(17, 52, 0).expect("Time Conversion failed")
        );
    }

    #[test]
    fn time_default() {
        let time = Time::default();
        match time {
            Time::Hm {
                hour,
                minute,
                period,
            } => {
                assert_eq!(hour, 1);
                assert_eq!(minute, 0);
                assert_eq!(period, Period::Am);
            }
            _ => panic!("Expected Time::Hm variant"),
        }
    }

    #[test]
    fn time_default_hm_am() {
        let time = Time::default_hm(Period::Am);
        match time {
            Time::Hm {
                hour,
                minute,
                period,
            } => {
                assert_eq!(hour, 0);
                assert_eq!(minute, 0);
                assert_eq!(period, Period::Am);
            }
            _ => panic!("Expected Time::Hm variant"),
        }
    }

    #[test]
    fn time_default_hm_pm() {
        let time = Time::default_hm(Period::Pm);
        match time {
            Time::Hm {
                hour,
                minute,
                period,
            } => {
                assert_eq!(hour, 0);
                assert_eq!(minute, 0);
                assert_eq!(period, Period::Pm);
            }
            _ => panic!("Expected Time::Hm variant"),
        }
    }

    #[test]
    fn time_default_hm_h24() {
        let time = Time::default_hm(Period::H24);
        match time {
            Time::Hm {
                hour,
                minute,
                period,
            } => {
                assert_eq!(hour, 0);
                assert_eq!(minute, 0);
                assert_eq!(period, Period::H24);
            }
            _ => panic!("Expected Time::Hm variant"),
        }
    }

    #[test]
    fn time_default_hms_am() {
        let time = Time::default_hms(Period::Am);
        match time {
            Time::Hms {
                hour,
                minute,
                second,
                period,
            } => {
                assert_eq!(hour, 0);
                assert_eq!(minute, 0);
                assert_eq!(second, 0);
                assert_eq!(period, Period::Am);
            }
            _ => panic!("Expected Time::Hms variant"),
        }
    }

    #[test]
    fn time_default_hms_pm() {
        let time = Time::default_hms(Period::Pm);
        match time {
            Time::Hms {
                hour,
                minute,
                second,
                period,
            } => {
                assert_eq!(hour, 0);
                assert_eq!(minute, 0);
                assert_eq!(second, 0);
                assert_eq!(period, Period::Pm);
            }
            _ => panic!("Expected Time::Hms variant"),
        }
    }

    #[test]
    fn time_default_hms_h24() {
        let time = Time::default_hms(Period::H24);
        match time {
            Time::Hms {
                hour,
                minute,
                second,
                period,
            } => {
                assert_eq!(hour, 0);
                assert_eq!(minute, 0);
                assert_eq!(second, 0);
                assert_eq!(period, Period::H24);
            }
            _ => panic!("Expected Time::Hms variant"),
        }
    }

    #[test]
    fn time_hm_display_h24() {
        let time = Time::Hm {
            hour: 14,
            minute: 30,
            period: Period::H24,
        };
        assert_eq!(format!("{}", time), "14:30");
    }

    #[test]
    fn time_hm_display_am() {
        let time = Time::Hm {
            hour: 9,
            minute: 15,
            period: Period::Am,
        };
        assert_eq!(format!("{}", time), "09:15 AM");
    }

    #[test]
    fn time_hm_display_pm() {
        let time = Time::Hm {
            hour: 3,
            minute: 45,
            period: Period::Pm,
        };
        assert_eq!(format!("{}", time), "03:45 PM");
    }

    #[test]
    fn time_hms_display_h24() {
        let time = Time::Hms {
            hour: 23,
            minute: 59,
            second: 59,
            period: Period::H24,
        };
        assert_eq!(format!("{}", time), "23:59:59");
    }

    #[test]
    fn time_hms_display_am() {
        let time = Time::Hms {
            hour: 6,
            minute: 30,
            second: 45,
            period: Period::Am,
        };
        assert_eq!(format!("{}", time), "06:30:45 AM");
    }

    #[test]
    fn time_hms_display_pm() {
        let time = Time::Hms {
            hour: 11,
            minute: 22,
            second: 33,
            period: Period::Pm,
        };
        assert_eq!(format!("{}", time), "11:22:33 PM");
    }

    #[test]
    fn period_display_h24() {
        assert_eq!(format!("{}", Period::H24), "");
    }

    #[test]
    fn period_display_am() {
        assert_eq!(format!("{}", Period::Am), " AM");
    }

    #[test]
    fn period_display_pm() {
        assert_eq!(format!("{}", Period::Pm), " PM");
    }

    #[test]
    fn naive_time_to_time() {
        let naive = NaiveTime::from_hms_opt(14, 30, 45).unwrap();
        let time: Time = naive.into();
        match time {
            Time::Hms {
                hour,
                minute,
                second,
                period,
            } => {
                assert_eq!(hour, 14);
                assert_eq!(minute, 30);
                assert_eq!(second, 45);
                assert_eq!(period, Period::H24);
            }
            _ => panic!("Expected Time::Hms variant"),
        }
    }

    #[test]
    fn naive_time_midnight_to_time() {
        let naive = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let time: Time = naive.into();
        match time {
            Time::Hms {
                hour,
                minute,
                second,
                period,
            } => {
                assert_eq!(hour, 0);
                assert_eq!(minute, 0);
                assert_eq!(second, 0);
                assert_eq!(period, Period::H24);
            }
            _ => panic!("Expected Time::Hms variant"),
        }
    }

    #[test]
    fn time_hm_midnight_am_to_naive() {
        let time = Time::Hm {
            hour: 12,
            minute: 0,
            period: Period::Am,
        };
        let naive: NaiveTime = time.into();
        assert_eq!(naive, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    }

    #[test]
    fn time_hm_noon_pm_to_naive() {
        let time = Time::Hm {
            hour: 12,
            minute: 0,
            period: Period::Pm,
        };
        let naive: NaiveTime = time.into();
        assert_eq!(naive, NaiveTime::from_hms_opt(12, 0, 0).unwrap());
    }

    #[test]
    fn time_hm_one_am_to_naive() {
        let time = Time::Hm {
            hour: 1,
            minute: 30,
            period: Period::Am,
        };
        let naive: NaiveTime = time.into();
        assert_eq!(naive, NaiveTime::from_hms_opt(1, 30, 0).unwrap());
    }

    #[test]
    fn time_hm_eleven_pm_to_naive() {
        let time = Time::Hm {
            hour: 11,
            minute: 45,
            period: Period::Pm,
        };
        let naive: NaiveTime = time.into();
        assert_eq!(naive, NaiveTime::from_hms_opt(23, 45, 0).unwrap());
    }

    #[test]
    fn time_copy_trait() {
        let time1 = Time::Hm {
            hour: 10,
            minute: 30,
            period: Period::Am,
        };
        let time2 = time1; // Copy
        // Both should be usable
        match time1 {
            Time::Hm { hour, .. } => assert_eq!(hour, 10),
            _ => panic!("Expected Time::Hm"),
        }
        match time2 {
            Time::Hm { hour, .. } => assert_eq!(hour, 10),
            _ => panic!("Expected Time::Hm"),
        }
    }

    #[test]
    fn time_clone_trait() {
        let time1 = Time::Hms {
            hour: 15,
            minute: 45,
            second: 30,
            period: Period::H24,
        };
        let time2 = time1.clone();
        match (time1, time2) {
            (
                Time::Hms {
                    hour: h1,
                    minute: m1,
                    second: s1,
                    period: p1,
                },
                Time::Hms {
                    hour: h2,
                    minute: m2,
                    second: s2,
                    period: p2,
                },
            ) => {
                assert_eq!(h1, h2);
                assert_eq!(m1, m2);
                assert_eq!(s1, s2);
                assert_eq!(p1, p2);
            }
            _ => panic!("Expected Time::Hms for both"),
        }
    }

    #[test]
    fn period_copy_trait() {
        let p1 = Period::Am;
        let p2 = p1; // Copy
        assert_eq!(p1, Period::Am);
        assert_eq!(p2, Period::Am);
    }

    #[test]
    fn period_clone_trait() {
        let p1 = Period::Pm;
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }

    #[test]
    fn period_equality() {
        assert_eq!(Period::Am, Period::Am);
        assert_eq!(Period::Pm, Period::Pm);
        assert_eq!(Period::H24, Period::H24);
        assert_ne!(Period::Am, Period::Pm);
        assert_ne!(Period::Am, Period::H24);
        assert_ne!(Period::Pm, Period::H24);
    }
}
