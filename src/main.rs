#[cfg(test)]
mod test; 

mod constants;
mod timer;
use crate::constants::{WORK_BLOCK_PROMPT, BREAK_BLOCK_PROMPT, NUM_SESSIONS_PROMPT};
use crate::timer::start_timer;
use std::io;

fn main() {
    let work_dur: u64;
    let break_dur: u64;
    let sessions: u64;
    (work_dur, break_dur, sessions) = input_mask();
    for _ in 0..sessions {
        // Start Work Timer
        start_timer(work_dur as f64, true);

        // Start Break Timer
        start_timer(break_dur as f64, true);
    }
}

fn get_value(prompt: &str, default: u64) -> u64 {
    let mut buffer: String = String::new();
    let return_value: u64;
    println!("{}", prompt);
    io::stdin().read_line(&mut buffer).ok();
    match buffer.trim().parse::<u64>() {
        Ok(value) => {
            return_value = value;
        }
        Err(_) => {
            let message: String = format!("Failed to parse.. Using default {}!", default.to_string());
            println!("{}", message);
            return_value = default;
        }
    }
    return return_value;
}

fn input_mask() -> (u64, u64, u64) {
    let dur_work_block: u64 = get_value(WORK_BLOCK_PROMPT, 25);
    let dur_break_block: u64 = get_value(BREAK_BLOCK_PROMPT, 5);
    let num_sessions: u64 = get_value(NUM_SESSIONS_PROMPT, 4);

    return (dur_work_block, dur_break_block, num_sessions);
}