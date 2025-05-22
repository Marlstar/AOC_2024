use crate::get_input_lines;

pub fn run() {
    let d2 = Day2::new();
    println!("Part 1: {}\nPart 2: {}", d2.part1(), d2.part2());
}

pub struct Day2 {
    reports: Vec<Vec<isize>>
}
impl Day2 {
    pub fn new() -> Self {
        let line = |a: &str| -> Vec<isize> { return a.split(" ").map(|a| a.parse::<isize>().unwrap()).collect::<Vec<isize>>(); };
        return Self {
            reports: get_input_lines("02").iter().map(|l| line(l)).collect()
        }
    }
    pub fn part1(&self) -> usize {
        let mut safe = 0usize;
        for report in &self.reports {
            if Self::valid(report) { safe += 1; }
        }
        return safe;
    }

    pub fn part2(&self) -> usize {
        let mut safe = 0usize;
        for report in &self.reports {
            if Self::valid(report) { safe += 1; continue; }
            'f: for (i,_) in report.iter().enumerate() {
                if Self::valid_with_skip(report, i) { safe += 1; break 'f; }
            }
        }
        return safe;
    }

    fn valid_with_skip(report: &[isize], skip: usize) -> bool {
        let shortened: Vec<isize> = report.iter().enumerate().filter(|(i, _)| i != &skip).map(|(_,a)| a).copied().collect();
        return Self::valid(&shortened)
    }

    fn valid(report: &[isize]) -> bool {
        let mut ab = vec![0; report.len()];
        ab.clone_from_slice(report);
        ab.sort();
        ab.dedup();
        let mut i1 = ab.iter();
        let mut i2 = ab.iter().skip(1);
        let mut k = true;
        for n2 in i2 {
            let n1 = i1.next().unwrap();
            let diff = n2 - n1;
            if diff > 3 || diff < 1 { k = false }
        }
        let bb: Vec<isize> = ab.iter().rev().copied().collect();
        return (report == bb || report == ab) && ab.len() == report.len() && k;
    }
}
