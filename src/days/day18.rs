use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day18_test.txt"), 62);
    dbg!(part1("assets/day18.txt"));
    assert_eq!(part2("assets/day18_test.txt"), 952408144115);
    dbg!(part2("assets/day18.txt"));
}

fn part2(file: &str) -> u64 {
    let plans = parse_file(file, true);
    solve(&plans)
}

fn part1(file: &str) -> u64 {
    let plans = parse_file(file, false);
    solve(&plans)
}

fn solve(plans: &[Plan]) -> u64 {
    let mut points = Vec::new();
    points.push(Vec2::new(0, 0));
    for plan in plans {
        let last = points.last().unwrap();
        let next = last + plan.dir.to_vec() * plan.dist as i32;
        points.push(next);
    }

    let points = points
        .into_iter()
        .map(|p| p.as_::<i64>())
        .collect::<Vec<_>>();

    let area = points
        .iter()
        .zip(points.iter().skip(1))
        .fold(0, |sum, (a, b)| sum + a.x * b.y - a.y * b.x);
    let per: u32 = plans.iter().map(|p| p.dist).sum();
    area.unsigned_abs() / 2 + per as u64 / 2 + 1
}

fn parse_file(file: &str, color: bool) -> Vec<Plan> {
    let file = read_file(file);
    file.lines()
        .map(|l| {
            let (a, b, c): (String, String, String);
            scan!(l.bytes() => "{} {} (#{})", a, b, c);
            if !color {
                let dir = match a.as_str() {
                    "U" => Dir::Up,
                    "D" => Dir::Down,
                    "L" => Dir::Left,
                    "R" => Dir::Right,
                    _ => unreachable!(),
                };
                let dist = b.parse::<u32>().unwrap();
                Plan { dir, dist }
            } else {
                let dir = match c.chars().nth(5).unwrap() {
                    '0' => Dir::Right,
                    '1' => Dir::Down,
                    '2' => Dir::Left,
                    '3' => Dir::Up,
                    _ => unreachable!(),
                };
                let dist = c.chars().take(5).collect::<String>();
                let dist = u32::from_str_radix(&dist, 16).unwrap();
                Plan { dir, dist }
            }
        })
        .collect::<Vec<_>>()
}

struct Plan {
    dir: Dir,
    dist: u32,
}
