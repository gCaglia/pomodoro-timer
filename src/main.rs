#[cfg(test)]
mod test; 

mod constants;
mod timer;
mod sound_player;

use std::io;
use std::fs::File;
use crate::constants::{WORK_BLOCK_PROMPT, BREAK_BLOCK_PROMPT, NUM_SESSIONS_PROMPT};
use crate::timer::start_timer;
use crate::sound_player::{SoundsLibrary, Player};

fn main() {
    let work_dur: u64;
    let break_dur: u64;
    let sessions: u64;
    (work_dur, break_dur, sessions) = input_mask();
    let mut work_sound_file: File;
    let mut library: SoundsLibrary;
    for session in 1..(sessions + 1) {
        // Prepare work sound
        work_sound_file = SoundsLibrary::open_file();
        library = SoundsLibrary::set_sounds(work_sound_file);

        // Start Work Timer
        library.play_sound();
        start_timer(work_dur as f64, true);

        // Start Break Timer if not on last session
        if session + 1 < session {
            start_timer(break_dur as f64, true);
        }
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