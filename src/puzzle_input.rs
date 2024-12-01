use std::io::{prelude::*, BufReader};
fn file(day: usize) -> std::fs::File {
    return std::fs::File::open(format!("input/{day}.txt")).expect("failed to open input file");
}
pub fn get_input_lines(day: usize) -> Vec<String> {
    let reader = BufReader::new(file(day));
    let mut out: Vec<String> = vec![];
    for line in reader.lines() {
        out.push(line.unwrap());
    }
    return out;
}

pub fn get_input_raw(day: usize) -> String {
    let mut out = String::new();
    return file(day).read_to_string(&mut out).expect("failed to read input to string").to_string();
}
