#[cfg(test)]
mod test; 

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

fn get_value(prompt: String, default: i8) -> i8 {
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
    let work_block_prompt: String = "Set work block duration:".to_owned();
    let break_block_prompt: String = "Set break duration:".to_owned();
    let num_sessions_prompt: String = "Set number of sessions:".to_owned();
    let dur_work_block: i8 = get_value(work_block_prompt, 25);
    let dur_break_block: i8 = get_value(break_block_prompt, 5);
    let num_sessions: i8 = get_value(num_sessions_prompt, 4);

    return (dur_work_block, dur_break_block, num_sessions);
}