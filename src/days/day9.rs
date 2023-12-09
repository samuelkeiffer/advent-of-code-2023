use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day9_test.txt"), 114);
    dbg!(part1("assets/day9.txt"));
    assert_eq!(part2("assets/day9_test.txt"), 2);
    dbg!(part2("assets/day9.txt"));
}

fn part2(file: &str) -> i32 {
    let histories = parse_file(file);
    histories.iter().map(|h| h.extrapolate_back()).sum()
}

fn part1(file: &str) -> i32 {
    let histories = parse_file(file);
    histories.iter().map(|h| h.extrapolate()).sum()
}

fn parse_file(file: &str) -> Vec<History> {
    let file = read_file(file);
    file.lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        })
        .map(History)
        .collect()
}

impl History {
    fn extrapolate_back(&self) -> i32 {
        if self.0.iter().all(|x| *x == 0) {
            0
        } else {
            self.0.first().unwrap() - self.inner_hist().extrapolate_back()
        }
    }

    fn extrapolate(&self) -> i32 {
        if self.0.iter().all(|x| *x == 0) {
            0
        } else {
            self.0.last().unwrap() + self.inner_hist().extrapolate()
        }
    }

    fn inner_hist(&self) -> Self {
        Self(
            self.0
                .iter()
                .zip(self.0.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect(),
        )
    }
}

struct History(Vec<i32>);
