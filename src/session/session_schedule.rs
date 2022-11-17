use chrono::naive::Days;
use chrono::{DateTime, Datelike, FixedOffset, NaiveDateTime, NaiveTime, TimeZone, Utc, Weekday};
use chrono_tz::Tz;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum SessionSchedule {
    NonStop,
    Weekly {
        start_day: Weekday,
        end_day: Weekday,
        start_time: NaiveTime,
        end_time: NaiveTime,
        timezone: Option<Tz>,
        use_localtime: bool,
    },
    Daily {
        start_time: NaiveTime,
        end_time: NaiveTime,
        timezone: Option<Tz>,
        use_localtime: bool,
    },
}

// #[derive(Debug)]
// pub(crate) struct SessionSchedule {
//     non_stop_session: bool,
//     start_time: u32,
//     end_time: u32,
//     weekly_session: bool,
//     start_day: Option<Weekday>,
//     end_day: Option<Weekday>,
//     use_local_timezone: bool,
//     timezone: Option<u32>,
// }

impl SessionSchedule {
    pub const NON_STOP: Self = Self::NonStop;

    pub(crate) fn is_new_session(&self, old_time: DateTime<Utc>, test_time: DateTime<Utc>) -> bool {
        match self {
            SessionSchedule::NonStop => false,
            _ => {
                if old_time < test_time {
                    let next_end = self.next_end(old_time);
                    return old_time <= next_end && next_end < test_time;
                }
                return false;
            }
        }
    }

    //TODO convert to chrono::DateTime
    fn next_end(&self, old_time: DateTime<Utc>) -> DateTime<Utc> {
        assert!(self != &Self::NonStop);
        match self {
            SessionSchedule::NonStop => unreachable!(),
            SessionSchedule::Weekly { end_day, .. } => {
                let mut end = old_time.clone();
                let d = old_time;
                while &end.weekday() != end_day {
                    end = end + Days::new(1);
                }
                if d > end {
                    // d is later than end
                    end = end + Days::new(7);
                }
                end
            }
            SessionSchedule::Daily { .. } => {
                let mut end = old_time.clone();
                let d = old_time;
                if d > end {
                    // d is later than end
                    end = end + Days::new(1);
                }
                end
            }
        }
    }

    pub(crate) fn is_session_time(&self, time: &DateTime<Utc>) -> bool {
        // if (utc.Kind != System.DateTimeKind.Utc)
        //     throw new System.ArgumentException("Only UTC time is supported", "time");

        // System.DateTime adjusted = AdjustUtcDateTime(utc);

        // if (WeeklySession)
        //     return CheckDay(adjusted);
        // else
        //     return CheckTime(adjusted.TimeOfDay);
        let now = self.adjust_utc_datetime(*time);
        match self {
            SessionSchedule::NonStop => true,
            SessionSchedule::Weekly { .. } => self.check_day(now),
            SessionSchedule::Daily { .. } => self.check_time(&now.time()),
        }
    }

    fn check_day(&self, datetime: NaiveDateTime) -> bool {
        assert!(matches!(self, SessionSchedule::Weekly { .. }));
        match self {
            SessionSchedule::NonStop => unreachable!(),
            SessionSchedule::Weekly {
                start_day,
                end_day,
                start_time,
                end_time,
                ..
            } => {
                if start_day.num_days_from_monday() < end_day.num_days_from_monday() {
                    if datetime.weekday().num_days_from_monday() < start_day.num_days_from_monday()
                        || datetime.weekday().num_days_from_monday()
                            > end_day.num_days_from_monday()
                    {
                        false
                    } else if datetime.weekday().num_days_from_monday()
                        < end_day.num_days_from_monday()
                    {
                        start_day.num_days_from_monday() < datetime.weekday().num_days_from_monday()
                            || start_time <= &datetime.time()
                    } else {
                        end_day.num_days_from_monday() > datetime.weekday().num_days_from_monday()
                            || end_time >= &datetime.time()
                    }
                } else if end_day.num_days_from_monday() > start_day.num_days_from_monday() {
                    if datetime.weekday().num_days_from_monday() < end_day.num_days_from_monday()
                        || datetime.weekday().num_days_from_monday()
                            > start_day.num_days_from_monday()
                    {
                        false
                    } else if datetime.weekday().num_days_from_monday()
                        < start_day.num_days_from_monday()
                    {
                        end_day.num_days_from_monday() < datetime.weekday().num_days_from_monday()
                            || end_time <= &datetime.time()
                    } else {
                        start_day.num_days_from_monday() > datetime.weekday().num_days_from_monday()
                            || start_time >= &datetime.time()
                    }
                } else if start_time >= end_time {
                    &datetime.weekday() != start_day || self.check_time(&datetime.time())
                } else {
                    &datetime.weekday() == start_day && self.check_time(&datetime.time())
                }
            }
            SessionSchedule::Daily { .. } => unreachable!(),
        }
    }

    fn check_time(&self, time: &NaiveTime) -> bool {
        assert!(
            matches!(self, SessionSchedule::Daily { .. })
                || matches!(self, SessionSchedule::Weekly { .. })
        );
        let (start_time, end_time) = match self {
            SessionSchedule::NonStop => unreachable!(),
            SessionSchedule::Weekly {
                start_time,
                end_time,
                timezone,
                ..
            } => (start_time, start_time),
            SessionSchedule::Daily {
                start_time,
                end_time,
                timezone,
                ..
            } => (start_time, end_time),
        };

        if start_time < end_time {
            time >= start_time && time <= end_time
        } else if end_time < start_time {
            time >= start_time || time <= end_time
        } else {
            true
        }
    }

    fn adjust_utc_datetime(&self, now: DateTime<Utc>) -> NaiveDateTime {
        if self.use_local_time() {
            //return utc.ToLocalTime();
            now.naive_local()
        } else if let Some(timezone) = self.timezone() {
            // return System.TimeZoneInfo.ConvertTimeFromUtc(utc, TimeZone);
            now.with_timezone(timezone).naive_local()
        } else {
            now.naive_utc()
        }
    }

    fn use_local_time(&self) -> bool {
        match self {
            SessionSchedule::NonStop => false,
            SessionSchedule::Weekly { use_localtime, .. } => *use_localtime,
            SessionSchedule::Daily { use_localtime, .. } => *use_localtime,
        }
    }

    fn timezone(&self) -> Option<&Tz> {
        match self {
            SessionSchedule::NonStop => None,
            SessionSchedule::Weekly { timezone, .. } => timezone.as_ref(),
            SessionSchedule::Daily { timezone, .. } => timezone.as_ref(),
        }
    }
}
