use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day14_test.txt"), 136);
    dbg!(part1("assets/day14.txt"));
    assert_eq!(part2("assets/day14_test.txt"), 64);
    dbg!(part2("assets/day14.txt"));
}

fn part2(file: &str) -> u64 {
    let mut grid = parse_file(file);
    let mut vec = Vec::new();
    for i in 0..1000000000 {
        let init = grid.clone();
        grid = inverted_indices(&grid);
        roll_platform(&mut grid);
        grid = inverted_indices(&grid);
        roll_platform(&mut grid);
        grid = mirrored_indices(&inverted_indices(&grid));
        roll_platform(&mut grid);
        grid = mirrored_indices(&inverted_indices(&grid));
        roll_platform(&mut grid);
        grid = mirrored_indices(&inverted_indices(&mirrored_indices(&inverted_indices(
            &grid,
        ))));
        if init
            .iter()
            .zip(grid.iter())
            .all(|(r1, r2)| r1.iter().zip(r2.iter()).all(|(e1, e2)| e1 == e2))
        {
            break;
        }
        let test = inverted_indices(&grid);
        let x = test
            .iter()
            .map(|r| {
                r.iter()
                    .rev()
                    .enumerate()
                    .rev()
                    .filter_map(|(i, e)| matches!(e, Some(Rock::Round)).then_some(i + 1))
                    .sum::<usize>()
            })
            .sum::<usize>() as u64;
        vec.push(x);
        if vec.len() > 1000 {
            let a = vec.len() - 11;
            for j in 1..20 {
                let b = a - j;
                let c = a - j * 7;
                let d = a - j * 23;
                if vec[a..]
                    .iter()
                    .zip(vec[b..].iter())
                    .all(|(e1, e2)| e1 == e2)
                    && vec[a..]
                        .iter()
                        .zip(vec[c..].iter())
                        .all(|(e1, e2)| e1 == e2)
                    && vec[a..]
                        .iter()
                        .zip(vec[d..].iter())
                        .all(|(e1, e2)| e1 == e2)
                {
                    let offset = (1000000000 - 1 - i) % j;
                    return vec[i - j + offset];
                }
            }
        }
    }
    0
}

fn part1(file: &str) -> u64 {
    let grid = parse_file(file);
    let mut grid = inverted_indices(&grid);
    roll_platform(&mut grid);

    grid.iter()
        .map(|r| {
            r.iter()
                .rev()
                .enumerate()
                .rev()
                .filter_map(|(i, e)| matches!(e, Some(Rock::Round)).then_some(i + 1))
                .sum::<usize>()
        })
        .sum::<usize>() as u64
}

fn roll_platform(grid: &mut [Vec<Option<Rock>>]) {
    for row in grid.iter_mut() {
        let mut left = 0;
        for i in 0..row.len() {
            match row[i] {
                Some(Rock::Square) => {
                    left = i + 1;
                }
                Some(Rock::Round) => {
                    row.swap(left, i);
                    left += 1;
                }
                None => {}
            }
        }
    }
}

fn parse_file(file: &str) -> Vec<Vec<Option<Rock>>> {
    let file = read_file(file);
    file.lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Some(Rock::Square),
                    'O' => Some(Rock::Round),
                    '.' => None,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Rock {
    Round,
    Square,
}
