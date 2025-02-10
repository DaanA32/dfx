use chrono::naive::Days;
use chrono::{DateTime, Datelike, Duration, NaiveDateTime, NaiveTime, Timelike, Utc, Weekday};
use chrono_tz::Tz;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum SessionSchedule {
    NonStop,
    // #[cfg(test)]
    EvenMinutes,
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

impl SessionSchedule {
    pub(crate) fn is_new_session(&self, old_time: DateTime<Utc>, test_time: DateTime<Utc>) -> bool {
        match self {
            SessionSchedule::NonStop => false,
            _ => {
                if old_time < test_time {
                    let next_end = self.next_end(old_time);
                    return old_time <= next_end && next_end < test_time;
                }
                false
            }
        }
    }

    //TODO convert to chrono::DateTime
    fn next_end(&self, old_time: DateTime<Utc>) -> DateTime<Utc> {
        assert!(self != &Self::NonStop);
        match self {
            SessionSchedule::NonStop => unreachable!(),
            // #[cfg(test)]
            SessionSchedule::EvenMinutes => {
                let mut end = old_time;
                if end.minute() % 2 == 1 {
                    end += Duration::minutes(1);
                }
                end += Duration::minutes(2);
                end
            }
            SessionSchedule::Weekly { end_day, .. } => {
                let mut end = old_time;
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
                let mut end = old_time;
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
        let now = self.adjust_utc_datetime(*time);
        match self {
            SessionSchedule::NonStop => true,
            // #[cfg(test)]
            SessionSchedule::EvenMinutes => now.minute() % 2 == 0,
            SessionSchedule::Weekly { .. } => self.check_day(now),
            SessionSchedule::Daily { .. } => self.check_time(&now.time()),
        }
    }

    fn check_day(&self, datetime: NaiveDateTime) -> bool {
        assert!(matches!(self, SessionSchedule::Weekly { .. }));
        match self {
            SessionSchedule::NonStop => unreachable!(),
            // #[cfg(test)]
            SessionSchedule::EvenMinutes => unreachable!(),
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
            // #[cfg(test)]
            SessionSchedule::EvenMinutes => unreachable!(),
            SessionSchedule::Weekly {
                start_time,
                end_time,
                ..
            } => (start_time, end_time),
            SessionSchedule::Daily {
                start_time,
                end_time,
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
            now.naive_local()
        } else if let Some(timezone) = self.timezone() {
            now.with_timezone(timezone).naive_local()
        } else {
            now.naive_utc()
        }
    }

    fn use_local_time(&self) -> bool {
        match self {
            SessionSchedule::NonStop => false,
            // #[cfg(test)]
            SessionSchedule::EvenMinutes => false,
            SessionSchedule::Weekly { use_localtime, .. } => *use_localtime,
            SessionSchedule::Daily { use_localtime, .. } => *use_localtime,
        }
    }

    fn timezone(&self) -> Option<&Tz> {
        match self {
            SessionSchedule::NonStop => None,
            // #[cfg(test)]
            SessionSchedule::EvenMinutes => None,
            SessionSchedule::Weekly { timezone, .. } => timezone.as_ref(),
            SessionSchedule::Daily { timezone, .. } => timezone.as_ref(),
        }
    }
}
