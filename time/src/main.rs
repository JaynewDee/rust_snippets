use chrono::prelude::*;
fn main() {
    println!("Hello, world!");

    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();
    println!("UTC: {:?}\nLOCAL: {:?}", utc, local);

    // w/ year, month, day and hours, minutes, seconds
    let date_string = Utc.with_ymd_and_hms(1992, 9, 29, 3, 15, 0).unwrap();
    println!("TIME OF BIRTH: {:?}", date_string);
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use std::time::{Duration, Instant, SystemTime, SystemTimeError, TryFromFloatSecsError};

    #[test]
    fn test_durations_time() {
        let seconds = Duration::from_secs(7);
        let millis = Duration::from_millis(7_000);
        let micros = Duration::from_micros(7_000_000);
        let nanos = Duration::from_nanos(7_000_000_000);
        assert!(seconds == nanos);
        assert!(millis == micros);
        assert!(nanos == millis);
    }

    #[test]
    fn test_dates_chrono() {
        let utc: DateTime<Utc> = Utc::now();
        let local: DateTime<Local> = Local::now();
        assert_ne!(utc, local);
    }
}
