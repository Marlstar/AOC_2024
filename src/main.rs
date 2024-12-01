use std::io::{stdin,stdout,Write};
use chrono::{self,Datelike};
use std::time::Instant;
use colored::Colorize;

fn main() {
    let mut day_str = String::new();
    let current_date = chrono::Utc::now();
    let mut args = std::env::args();

    if let Some(next) = args.next() {
        day_str = next;
    } else {
        print!("{}", "DAY > ".blue());
        let _ = stdout().flush();
        match stdin().read_line(&mut day_str) {
            Ok(_) => {},
            Err(_) => println!("{}", "Invalid day, defaulting to today".red())
        };
    }
    let mut day = match day_str.trim().parse::<usize>() {
        Ok(a) => a,
        Err(_) => current_date.day() as usize
    };

    if day > 25 { day = 25; }

    let start_time = Instant::now();
    println!("{}", format!("<=== Running Day {day} ===>").green());
    match day {
        1 => aoc_2024::day_01::run(),
        2 => aoc_2024::day_02::run(),
        3 => aoc_2024::day_03::run(),
        4 => aoc_2024::day_04::run(),
        5 => aoc_2024::day_05::run(),
        6 => aoc_2024::day_06::run(),
        7 => aoc_2024::day_07::run(),
        8 => aoc_2024::day_08::run(),
        9 => aoc_2024::day_09::run(),
        10 => aoc_2024::day_10::run(),
        11 => aoc_2024::day_11::run(),
        12 => aoc_2024::day_12::run(),
        13 => aoc_2024::day_13::run(),
        14 => aoc_2024::day_14::run(),
        15 => aoc_2024::day_15::run(),
        16 => aoc_2024::day_16::run(),
        17 => aoc_2024::day_17::run(),
        18 => aoc_2024::day_18::run(),
        19 => aoc_2024::day_19::run(),
        20 => aoc_2024::day_20::run(),
        21 => aoc_2024::day_21::run(),
        22 => aoc_2024::day_22::run(),
        23 => aoc_2024::day_23::run(),
        24 => aoc_2024::day_24::run(),
        25 => aoc_2024::day_25::run(),
        _ => {}
    }

    let time_taken_str = format!("{:?}", start_time.elapsed()).red();
    println!("{}{time_taken_str}{}", "<=== Took ".green(), " ===>".green());
}

