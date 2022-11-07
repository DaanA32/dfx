use chrono::{DateTime, Utc, Weekday};

#[derive(Debug)]
pub struct SessionSchedule {
    non_stop_session: bool,
    start_time: u32,
    end_time: u32,
    weekly_session: bool,
    start_day: Option<Weekday>,
    end_day: Option<Weekday>,
    use_local_timezone: bool,
    timezone: Option<u32>
}

impl SessionSchedule {
    pub(crate) fn is_new_session(&self, old_time: DateTime<Utc>, test_time: DateTime<Utc>) -> bool {
        if self.non_stop_session {
            return false;
        }
        if old_time < test_time {
            let next_end = self.next_end(old_time);
            return old_time <= next_end && next_end < test_time;
        }
        return false;
    }

    //TODO convert to chrono::DateTime
    fn next_end(&self, old_time: DateTime<Utc>) -> DateTime<Utc> {
        assert!(!self.non_stop_session);
        let mut  d = old_time;
        // DateTime d = AdjustUtcDateTime(utc);
        // DateTime end = DateTime.MinValue;

        let mut end = d.clone();
        if self.weekly_session {
            todo!()
        } else {
        //     end = new DateTime(d.Year, d.Month, d.Day, EndTime.Hours, EndTime.Minutes, EndTime.Seconds, d.Kind);
        //     if (DateTime.Compare(d, end) > 0) // d is later than end
        //         end = end.AddDays(1);
            todo!()
        }
        end
        // if (WeeklySession)
        // {
        // }
        // else
        // {
        // }
    }

    pub(crate) fn is_session_time(&self, now: DateTime<Utc>) -> bool {
        todo!()
    }
}
