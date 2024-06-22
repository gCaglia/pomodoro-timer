use std::time::Duration;
use std::thread::sleep;
use crate::constants::PRINT_INTERVAL;

pub fn start_timer(minutes: f64, count_down: bool) {
    let mut wait_seconds: u64 = (minutes * 60.0) as u64;
    let mut periods_waited: u64 = 0;
    let mut formatted_time: String;
    while wait_seconds > 0 {
        sleep(Duration::new(1 as u64, 0));
        periods_waited += 1;
        wait_seconds -= 1;
        if (periods_waited % PRINT_INTERVAL == 0) && count_down{
            formatted_time = format_seconds(wait_seconds);
            println!("Remaining Time: {}", formatted_time)
        }
    }

    sleep(Duration::new(wait_seconds as u64, 0));
}


pub fn format_seconds(seconds: u64) -> String {
    let mins: u64 = seconds / 60;
    let secs: u64 = seconds % 60;
    let formatted: String = format!("{}:{}", mins, secs);
    return formatted;
}