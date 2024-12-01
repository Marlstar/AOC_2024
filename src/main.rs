use std::io::{stdin,stdout,Write};
use chrono::{self,Datelike};

fn main() {
    let mut day = String::new();

    let current_date = chrono::Utc::now();

    print!("DAY > ");
    let _ = stdout().flush();
    match stdin().read_line(&mut day) {
        Ok(_) => {},
        Err(_) => { println!("Invalid day, defaulting"); day = String::from(""); }
    }
    let mut day = match day.trim() {
        "" => current_date.day() as usize,
        _ => day.trim().parse::<usize>().expect("failed to parse day to number")
    };
    if day > 25 { day = 25; }

    println!("<=== Running Day {day} ===>");
    match day {
        1 => aoc_2024::day_1::run(),
        2 => aoc_2024::day_2::run(),
        3 => aoc_2024::day_3::run(),
        4 => aoc_2024::day_4::run(),
        5 => aoc_2024::day_5::run(),
        6 => aoc_2024::day_6::run(),
        7 => aoc_2024::day_7::run(),
        8 => aoc_2024::day_8::run(),
        9 => aoc_2024::day_9::run(),
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
}

