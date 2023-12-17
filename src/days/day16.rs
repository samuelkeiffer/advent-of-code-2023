use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day16_test.txt"), 46);
    let handler = thread::Builder::new()
        .stack_size(2048 * 1024)
        .spawn(move || {
            dbg!(part1("assets/day16.txt"));
        })
        .unwrap();
    handler.join().unwrap();
    assert_eq!(part2("assets/day16_test.txt"), 51);
    let handler = thread::Builder::new()
        .stack_size(2048 * 1024)
        .spawn(move || {
            dbg!(part2("assets/day16.txt"));
        })
        .unwrap();
    handler.join().unwrap();
}

fn part2(file: &str) -> u64 {
    let grid = parse_file(file);
    let starting_beams = {
        let right = grid[0].len() - 1;
        let bottom = grid.len() - 1;
        (0..grid.len())
            .flat_map(|i| {
                [
                    (Vec2::new(i, 0), Dir::Right),
                    (Vec2::new(i, right), Dir::Left),
                ]
            })
            .chain((0..grid[0].len()).flat_map(|j| {
                [
                    (Vec2::new(0, j), Dir::Down),
                    (Vec2::new(bottom, j), Dir::Up),
                ]
            }))
            .collect::<Vec<_>>()
    };
    starting_beams
        .iter()
        .map(|(pos, dir)| {
            let mut beams = vec![vec![false; grid[0].len()]; grid.len()];
            let mut complete = HashSet::new();
            energize(&grid, &mut beams, &mut complete, *pos, *dir);
            beams
                .iter()
                .map(|r| r.iter().filter(|b| **b).count())
                .sum::<usize>() as u64
        })
        .max()
        .unwrap()
}

fn part1(file: &str) -> u64 {
    let grid = parse_file(file);
    let mut beams = vec![vec![false; grid[0].len()]; grid.len()];
    let mut complete = HashSet::new();
    energize(
        &grid,
        &mut beams,
        &mut complete,
        Vec2::new(0, 0),
        Dir::Right,
    );
    beams
        .iter()
        .map(|r| r.iter().filter(|b| **b).count())
        .sum::<usize>() as u64
}

fn parse_file(file: &str) -> Vec<Vec<Tile>> {
    let file = read_file(file);
    file.lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '/' => Tile::FMirror,
                    '\\' => Tile::BMirror,
                    '|' => Tile::VSplitter,
                    '-' => Tile::HSplitter,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn energize(
    grid: &[Vec<Tile>],
    beams: &mut [Vec<bool>],
    complete: &mut HashSet<(Vec2<usize>, Dir)>,
    pos: Vec2<usize>,
    dir: Dir,
) {
    if grid.get(pos.x).and_then(|r| r.get(pos.y)).is_none() || complete.contains(&(pos, dir)) {
        return;
    }
    beams[pos.x][pos.y] = true;
    complete.insert((pos, dir));
    match grid[pos.x][pos.y] {
        Tile::Empty => {
            if let Some(next_pos) = dir.next_pos(pos) {
                energize(grid, beams, complete, next_pos, dir);
            }
        }
        Tile::FMirror => {
            let new_dir = match dir {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Up,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Left,
            };
            if let Some(next_pos) = new_dir.next_pos(pos) {
                energize(grid, beams, complete, next_pos, new_dir);
            }
        }
        Tile::BMirror => {
            let new_dir = match dir {
                Dir::Up => Dir::Left,
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Right,
            };
            if let Some(next_pos) = new_dir.next_pos(pos) {
                energize(grid, beams, complete, next_pos, new_dir);
            }
        }
        Tile::VSplitter => match dir {
            Dir::Up | Dir::Down => {
                if let Some(next_pos) = dir.next_pos(pos) {
                    energize(grid, beams, complete, next_pos, dir);
                }
            }
            Dir::Left | Dir::Right => {
                if let Some(next_pos) = Dir::Up.next_pos(pos) {
                    energize(grid, beams, complete, next_pos, Dir::Up);
                }
                if let Some(next_pos) = Dir::Down.next_pos(pos) {
                    energize(grid, beams, complete, next_pos, Dir::Down);
                }
            }
        },
        Tile::HSplitter => match dir {
            Dir::Left | Dir::Right => {
                if let Some(next_pos) = dir.next_pos(pos) {
                    energize(grid, beams, complete, next_pos, dir);
                }
            }
            Dir::Up | Dir::Down => {
                if let Some(next_pos) = Dir::Left.next_pos(pos) {
                    energize(grid, beams, complete, next_pos, Dir::Left);
                }
                if let Some(next_pos) = Dir::Right.next_pos(pos) {
                    energize(grid, beams, complete, next_pos, Dir::Right);
                }
            }
        },
    }
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Empty,
    FMirror,
    BMirror,
    VSplitter,
    HSplitter,
}
