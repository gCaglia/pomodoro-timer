#[cfg(test)]
mod test; 

mod constants;
use std::io;

fn main() {
    let work_dur: i8;
    let break_dur: i8;
    let sessions: i8;
    (work_dur, break_dur, sessions) = input_mask();
    println!("Hello, world!");
    println!("Work Duration: {}", work_dur);
    println!("Break Duration: {}", break_dur);
    println!("Number of Sessions: {}", sessions);
}

fn get_value(prompt: &str, default: i8) -> i8 {
    let mut buffer: String = String::new();
    let return_value: i8;
    println!("{}", prompt);
    io::stdin().read_line(&mut buffer).ok();
    match buffer.trim().parse::<i8>() {
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

fn input_mask() -> (i8, i8, i8) {
    let dur_work_block: i8 = get_value(constants::WORK_BLOCK_PROMPT, 25);
    let dur_break_block: i8 = get_value(constants::BREAK_BLOCK_PROMPT, 5);
    let num_sessions: i8 = get_value(constants::NUM_SESSIONS_PROMPT, 4);

    return (dur_work_block, dur_break_block, num_sessions);
}