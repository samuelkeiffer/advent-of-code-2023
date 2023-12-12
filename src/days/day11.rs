use crate::*;

pub fn run() {
    assert_eq!(test("assets/day11_test.txt", 2), 374);
    dbg!(test("assets/day11.txt", 2));
    assert_eq!(test("assets/day11_test.txt", 10), 1030);
    assert_eq!(test("assets/day11_test.txt", 100), 8410);
    dbg!(test("assets/day11.txt", 1000000));
}

fn test(file: &str, exp: u64) -> u64 {
    let universe = parse_file(file, exp);
    let mut sum = 0;
    for i in 0..universe.galaxies.len() {
        for j in (i + 1)..universe.galaxies.len() {
            let a = universe.galaxies[i].as_::<i64>();
            let b = universe.galaxies[j].as_::<i64>();
            let x_dist = {
                let range = if a.x < b.x { a.x..b.x } else { b.x..a.x };
                (a.x - b.x).unsigned_abs()
                    + range
                        .filter(|x| universe.empty_rows.iter().any(|r| *r == *x as usize))
                        .count() as u64
                        * (exp - 1)
            };
            let y_dist = {
                let range = if a.y < b.y { a.y..b.y } else { b.y..a.y };
                (a.y - b.y).unsigned_abs()
                    + range
                        .filter(|y| universe.empty_cols.iter().any(|c| *c == *y as usize))
                        .count() as u64
                        * (exp - 1)
            };
            sum += x_dist + y_dist;
        }
    }
    sum
}

fn parse_file(file: &str, _exp: u64) -> Universe {
    let file = read_file(file);
    let mut galaxies = Vec::new();
    let grid = file
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| {
                    let gal = matches!(c, '#');
                    if gal {
                        galaxies.push(Vec2::new(i, j));
                    }
                    gal
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut empty_rows = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        if row.iter().all(|x| !x) {
            empty_rows.push(i);
        }
    }

    let mut empty_cols = Vec::new();
    for i in 0..grid[0].len() {
        let empty = grid.iter().map(|r| r[i]).all(|x| !x);
        if empty {
            empty_cols.push(i);
        }
    }

    Universe {
        galaxies,
        empty_rows,
        empty_cols,
    }
}

struct Universe {
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
    galaxies: Vec<Vec2<usize>>,
}
