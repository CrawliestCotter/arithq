use crate::quizzes::*;
use crate::tools::*;
pub fn menu() {
    loop {
        print!(
            "Choose a quiz:\n\
            (1) Addition,\t\t(2) Subtraction,\t(3) Multiplication,\n\
            (4) Div. Division,\t(5) Rem. Division,\t(6) Random,\n\
            (0) Quit: "
        );
        let number = get_number();
        match number {
            0 => break,
            6 => rand_test(),
            _ => loop_test(number as u32),
        }
    }
}

pub fn select_test(kind: u32) -> bool {
    let result = match kind {
        1 => add(),
        2 => sub(),
        3 => mult(),
        4 => int_div(),
        5 => rem_div(),
        _ => {
            println!("Invaid option");
            false
        }
    };
    result
}

pub fn loop_test(kind: u32) {
    let time = get_now();
    let mut i = 0;
    while select_test(kind) {
        i += 1;
    }
    if kind <= 5 {
        let time = get_now() - time;
        print_stats(i, time);
    }
}

pub fn rand_test() {
    let time = get_now();
    let mut i = 0;
    while select_test(get_rand(1, 4) as u32) {
        i += 1;
    }
    let time = get_now() - time;
    print_stats(i, time);
}

fn print_stats(i: u64, t: u64) {
    let apm = i as f64 * 60.0 / t as f64;
    println!(
        "{i} ops. in {t} seconds. Rate: {:.0} ops. per minute.\n",
        apm
    );
}
