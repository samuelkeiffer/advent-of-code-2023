use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day13_test.txt"), 405);
    dbg!(part1("assets/day13.txt"));
    assert_eq!(part2("assets/day13_test.txt"), 400);
    dbg!(part2("assets/day13.txt"));
}

fn part2(file: &str) -> u64 {
    let lands = parse_file(file, true);
    lands
        .iter()
        .map(|l| match l.refl_ori {
            Orientation::Vertical => l.relf_pos as u64 + 1,
            Orientation::Horizontal => (l.relf_pos as u64 + 1) * 100,
        })
        .sum()
}

fn part1(file: &str) -> u64 {
    let lands = parse_file(file, false);
    lands
        .iter()
        .map(|l| match l.refl_ori {
            Orientation::Vertical => l.relf_pos as u64 + 1,
            Orientation::Horizontal => (l.relf_pos as u64 + 1) * 100,
        })
        .sum()
}

fn parse_file(file: &str, smudge: bool) -> Vec<Land> {
    let file = read_file(file);
    let mut grid = Vec::new();
    let mut grids = Vec::new();
    for line in file.lines() {
        if line.is_empty() {
            let grid = take(&mut grid);
            grids.push(grid);
        } else {
            let row = line.chars().map(|c| matches!(c, '#')).collect::<Vec<_>>();
            grid.push(row);
        }
    }
    grids.push(grid);

    grids
        .iter()
        .map(|grid| {
            let (ori, i) = if smudge {
                find_smudge_refl(grid)
            } else {
                find_refl(grid)
            };
            Land {
                refl_ori: ori,
                relf_pos: i,
            }
        })
        .collect::<Vec<_>>()
}

fn find_smudge_refl(grid: &[Vec<bool>]) -> (Orientation, usize) {
    for i in 1..grid.len() {
        if grid[i..]
            .iter()
            .zip(grid[..i].iter().rev())
            .map(|(r1, r2)| r1.iter().zip(r2.iter()).filter(|(e1, e2)| e1 != e2).count())
            .sum::<usize>()
            == 1
        {
            return (Orientation::Horizontal, i - 1);
        }
    }

    let invert_grid = inverted_indices(grid);
    for i in 1..invert_grid.len() {
        if invert_grid[i..]
            .iter()
            .zip(invert_grid[..i].iter().rev())
            .map(|(r1, r2)| r1.iter().zip(r2.iter()).filter(|(e1, e2)| e1 != e2).count())
            .sum::<usize>()
            == 1
        {
            return (Orientation::Vertical, i - 1);
        }
    }

    unreachable!()
}

fn find_refl(grid: &[Vec<bool>]) -> (Orientation, usize) {
    for i in 1..grid.len() {
        if grid[i..]
            .iter()
            .zip(grid[..i].iter().rev())
            .all(|(r1, r2)| r1.iter().zip(r2.iter()).all(|(e1, e2)| e1 == e2))
        {
            return (Orientation::Horizontal, i - 1);
        }
    }

    let invert_grid = inverted_indices(grid);
    for i in 1..invert_grid.len() {
        if invert_grid[i..]
            .iter()
            .zip(invert_grid[..i].iter().rev())
            .all(|(r1, r2)| r1.iter().zip(r2.iter()).all(|(e1, e2)| e1 == e2))
        {
            return (Orientation::Vertical, i - 1);
        }
    }

    unreachable!()
}

struct Land {
    refl_ori: Orientation,
    relf_pos: usize,
}

#[derive(Copy, Clone, Debug)]
enum Orientation {
    Vertical,
    Horizontal,
}
