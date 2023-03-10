fn main() {
    println!("Hello, world!");

    //
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant, SystemTime, SystemTimeError, TryFromFloatSecsError};
    #[test]
    fn test_durations() {
        let seconds = Duration::from_secs(7);
        let millis = Duration::from_millis(7_000);
        let micros = Duration::from_micros(7_000_000);
        let nanos = Duration::from_nanos(7_000_000_000);
        assert!(seconds == nanos);
        assert!(millis == micros);
        assert!(nanos == millis);
    }
}
