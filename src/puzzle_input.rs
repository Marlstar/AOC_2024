use std::io::{prelude::*, BufReader};
fn file(day: &str) -> std::fs::File {
    return std::fs::File::open(format!("input/day_{day}.txt")).expect("failed to open input file");
}
pub fn get_input_lines(day: &str) -> Vec<String> {
    let reader = BufReader::new(file(day));
    let mut out: Vec<String> = vec![];
    for line in reader.lines() {
        out.push(line.unwrap());
    }
    return out;
}

pub fn get_input_raw(day: &str) -> String {
    let mut out = String::new();
    return file(day).read_to_string(&mut out).expect("failed to read input to string").to_string();
}
