use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day6_test.txt"), 288);
    dbg!(part1("assets/day6.txt"));
    assert_eq!(part2("assets/day6_test.txt"), 71503);
    dbg!(part2("assets/day6.txt"));
}

fn part2(file: &str) -> u64 {
    let file = read_file(file);
    let time = numeric_digits_from_line(&file, 0).parse::<u64>().unwrap();
    let record = numeric_digits_from_line(&file, 1).parse::<u64>().unwrap();
    (0..time)
        .filter(|hold| hold * (time - hold) > record)
        .count() as u64
}

fn part1(file: &str) -> u32 {
    let races = parse_file(file);
    races
        .iter()
        .map(|race| {
            (0..race.time)
                .filter(|hold| hold * (race.time - hold) > race.record)
                .count() as u32
        })
        .product()
}

fn parse_file(file: &str) -> Vec<Race> {
    let file = read_file(file);
    let times = file
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|t| t.parse::<u32>().ok());
    let records = file
        .lines()
        .nth(1)
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|t| t.parse::<u32>().ok());
    let mut races = Vec::new();
    for (time, record) in times.zip(records) {
        races.push(Race { time, record });
    }
    races
}

struct Race {
    time: u32,
    record: u32,
}
