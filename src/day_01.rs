use crate::get_input_lines;
use std::collections::HashMap;

pub fn run() {
    println!("Part 1: {}\nPart 2: {}", part1(), part2());
}

pub fn part1() -> usize {
    let mut input = get_input_lines("01");
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];
    for line in input {
        let s: Vec<usize> = line.split(" ").map(|a| {a.parse::<usize>().unwrap()}).collect();
        left.push(s[0]);
        right.push(s[1]);
    }
    left.sort();
    right.sort();

    let mut total = 0usize;

    let mut l = left.iter();
    let mut r = right.iter();
    let mut r_ = 0usize;

    for l_ in l {
        r_ = *r.next().unwrap();
        total += if l_ > &r_ { l_ - r_ } else { r_ - l_ };
    }

    return total;
}

pub fn part2() -> usize {
    let mut input = get_input_lines("01");
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];
    for line in input {
        let s: Vec<usize> = line.split(" ").map(|a| {a.parse::<usize>().unwrap()}).collect();
        left.push(s[0]);
        right.push(s[1]);
    }

    let mut nums: HashMap<usize, usize> = HashMap::new();
    for num in right {
        if let Some(a) = nums.get(&num) { nums.insert(num, a+1); }
        else { nums.insert(num,1); }
    }
    let mut total = 0usize;
    for num in left {
        if let Some(a) = nums.get(&num) { total += num * a; }
    }
    return total;
}

