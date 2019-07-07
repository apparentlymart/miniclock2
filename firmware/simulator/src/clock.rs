use bcdtime::DateTime;
use bcdtime::BCD;

pub struct SystemClock();

impl bcdtime::Read for SystemClock {
    type Error = !;

    fn read(&mut self) -> Result<DateTime, !> {
        use chrono::Datelike;
        use chrono::Timelike;
        let now = chrono::Local::now();
        Ok(DateTime {
            second: BCD::from(now.second() as u8),
            minute: BCD::from(now.minute() as u8),
            hour: BCD::from(now.hour() as u8),
            day: now.weekday().num_days_from_monday() as u8,
            date: BCD::from(now.day() as u8),
            month: BCD::from(now.month() as u8),
            year: BCD::from((now.year() - 2000) as u8),
            hr24: true,
        })
    }
}

impl bcdtime::Write for SystemClock {
    type Error = !;

    fn write(&mut self, _dt: &DateTime) -> Result<(), !> {
        // This currently does nothing, because we don't want to reset the
        // system clock.
        // Perhaps in future we'll change this type to track an offset from
        // the system clock so it can simulate the ability to set the clock
        // like on real hardware, but we don't yet have any way to set the
        // clock and so that is a moot point anyway.
        Ok(())
    }
}
