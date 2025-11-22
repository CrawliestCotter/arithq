use rand::Rng;
use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_rand(x: i32, y: i32) -> i32 {
    rand::rng().random_range(x..=y)
}

pub fn flush_print() {
    io::stdout().flush().expect("Failed to flush stdout");
}

pub fn get_number() -> i32 {
    let number: i32;
    loop {
        flush_print();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        number = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Enter a number: ");
                continue;
            }
        };
        break;
    }
    number
}

pub fn get_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time should go forward")
        .as_secs()
}
