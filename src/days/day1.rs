use crate::*;

pub fn run() {
    // assert_eq!(part1("assets/day1_test.txt"), 142);
    dbg!(part1("assets/day1.txt"));
    assert_eq!(part2("assets/day1_test.txt"), 281);
    dbg!(part2("assets/day1.txt"));
}

fn part2(file: &str) -> u32 {
    let calibration = parse_file_2(file);
    calibration.iter().sum()
}

fn part1(file: &str) -> u32 {
    let calibration = parse_file(file);
    calibration.iter().sum()
}

fn parse_file_2(file: &str) -> Vec<u32> {
    let calibration = read_file(file);
    calibration
        .lines()
        .filter_map(|line| {
            let line = line.replace("one", "on1e");
            let line = line.replace("two", "tw2o");
            let line = line.replace("three", "th3ree");
            let line = line.replace("four", "fo4ur");
            let line = line.replace("five", "fi5ve");
            let line = line.replace("six", "si6x");
            let line = line.replace("seven", "se7ven");
            let line = line.replace("eight", "ei8ght");
            let line = line.replace("nine", "ni9ne");
            let digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<_>>();
            let number = [*digits.first().unwrap(), *digits.last().unwrap()]
                .iter()
                .collect::<String>();
            number.parse::<u32>().ok()
        })
        .collect::<Vec<_>>()
}

fn parse_file(file: &str) -> Vec<u32> {
    let calibration = read_file(file);
    calibration
        .lines()
        .filter_map(|line| {
            let digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<_>>();
            let number = [*digits.first().unwrap(), *digits.last().unwrap()]
                .iter()
                .collect::<String>();
            number.parse::<u32>().ok()
        })
        .collect::<Vec<_>>()
}
