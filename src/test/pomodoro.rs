use crate::timer::{start_timer, format_seconds};
use std::time::{SystemTime, Duration, SystemTimeError};

#[test]
fn timer_expires_after_x_seconds() {
    let wait_time: f64 = 1.0/60.0;
    let expected_seconds: f64 = wait_time * 60.0;
    let start = SystemTime::now();
    start_timer(wait_time, false);
    let duration: Result<Duration, SystemTimeError> = start.elapsed();
    match duration {
        Ok(elapsed) => {
            assert_eq!(expected_seconds as u64, elapsed.as_secs());
        } 
        Err(e) => {
            println!("Unexpected error occuredL {}", e);
        }
    }
}

#[test]
fn test_format_seconds() {
    let seconds: u64 = 90;
    let actual: String = format_seconds(seconds);
    assert_eq!("1:30", actual);
}