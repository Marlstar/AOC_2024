use crate::get_input_lines;
use std::collections::HashMap;

const INPUT_SIZE: usize = 1000;

pub fn run() {
    let d = Day1::new();
    println!("Part 1: {}\nPart 2: {}", d.part1(), d.part2());
}

struct Day1 {
    left: [usize;INPUT_SIZE],
    right: [usize;INPUT_SIZE]
}
impl Day1 {
    pub fn new() -> Self {
        let input = get_input_lines("01");
        // there are 1000 lines in the input file
        let mut left: [usize;INPUT_SIZE] = [0;INPUT_SIZE];
        let mut right: [usize;INPUT_SIZE] = [0;INPUT_SIZE];
        let input_iter = input.iter().map(|a| {
            let mut parts = a.split(" ");
            (parts.next().unwrap().parse::<usize>(), parts.next().unwrap().parse::<usize>())
        });
        for (i, (l,r)) in input_iter.enumerate() {
            left[i] = l.unwrap();
            right[i] = r.unwrap();
        }

        left.sort();
        right.sort();

        return Self {
            left,
            right
        }
    }

    pub fn part1(&self) -> usize {
        let mut total = 0usize;

        let l = self.left.iter();
        let mut r = self.right.iter();
        let mut r_;

        for l_ in l {
            r_ = r.next().unwrap();
            // abs
            total += if l_ > r_ { l_ - r_ } else { r_ - l_ };
        }

        return total;
    }

    pub fn part2(&self) -> usize {
        let mut nums: HashMap<usize, usize> = HashMap::new();
        for num in &self.right {
            if let Some(a) = nums.get(&num) { nums.insert(*num, a+1); }
            else { nums.insert(*num,1); }
        }
        let mut total = 0usize;
        for num in &self.left {
            if let Some(a) = nums.get(num) { total += *num * a; }
        }

        return total;
    }
}
