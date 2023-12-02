use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day2_test.txt"), 8);
    dbg!(part1("assets/day2.txt"));
    assert_eq!(part2("assets/day2_test.txt"), 2286);
    dbg!(part2("assets/day2.txt"));
}

fn part2(file: &str) -> u32 {
    let games = parse_file(file);
    games
        .iter()
        .map(|game| {
            let mut min_pull = Pull {
                green: 0,
                red: 0,
                blue: 0,
            };
            game.iter().for_each(|pull| {
                min_pull.green = min_pull.green.max(pull.green);
                min_pull.red = min_pull.red.max(pull.red);
                min_pull.blue = min_pull.blue.max(pull.blue);
            });
            min_pull.green * min_pull.red * min_pull.blue
        })
        .sum()
}

fn part1(file: &str) -> u32 {
    let games = parse_file(file);
    games
        .iter()
        .enumerate()
        .filter(|(_i, game)| {
            !game
                .iter()
                .any(|pull| pull.red > 12 || pull.green > 13 || pull.blue > 14)
        })
        .map(|(i, _)| i as u32 + 1)
        .sum()
}

fn parse_file(file: &str) -> Vec<Vec<Pull>> {
    let file = read_file(file);
    file.lines()
        .map(|line| {
            let pulls = line.split(':').nth(1).unwrap();
            pulls
                .split(';')
                .map(|pull| pull.trim())
                .map(|pull| {
                    let set = pull.split(',').map(|set| set.trim());
                    let mut pull = Pull {
                        green: 0,
                        red: 0,
                        blue: 0,
                    };
                    for cubes in set {
                        let (num, kind) = cubes.split_once(' ').unwrap();
                        match kind {
                            "green" => pull.green = num.parse::<u32>().unwrap(),
                            "red" => pull.red = num.parse::<u32>().unwrap(),
                            "blue" => pull.blue = num.parse::<u32>().unwrap(),
                            _ => {}
                        }
                    }
                    pull
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[derive(Clone, Copy, Debug)]
struct Pull {
    green: u32,
    red: u32,
    blue: u32,
}
