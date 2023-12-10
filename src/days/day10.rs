use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day10_test.txt"), 8);
    dbg!(part1("assets/day10.txt"));
    assert_eq!(part2("assets/day10_test2.txt"), 10);
    dbg!(part2("assets/day10.txt"));
}

fn part2(file: &str) -> u32 {
    let (start_pos, grid) = parse_file(file);
    let mut pipes = HashSet::new();
    pipes.insert(start_pos);
    let mut check = start_pos;
    loop {
        let mut any = false;
        for pos in adjacent_pipes(&grid, check) {
            if pipes.insert(pos) {
                check = pos;
                any = true;
            }
        }
        if !any {
            break;
        }
    }
    let mut enclosed = vec![vec![false; grid[0].len()]; grid.len()];
    for i in 0..grid.len() {
        let mut crosses = 0;
        let mut n = false;
        let mut s = false;
        for j in 0..grid[0].len() {
            let pos = Vec2::new(i, j);
            let border = i == 0 || i == grid.len() - 1 || j == 0 || j == grid[0].len();
            if !border {
                if pipes.contains(&pos) {
                    if Pipe::north(&grid, pos) {
                        n = !n;
                    }
                    if Pipe::south(&grid, pos) {
                        s = !s;
                    }
                    if s && n {
                        crosses += 1;
                        n = false;
                        s = false;
                    }
                } else if crosses % 2 == 1 {
                    enclosed[i][j] = true;
                }
            }
        }
    }
    enclosed.iter().flatten().filter(|x| **x).count() as u32
}

fn part1(file: &str) -> u32 {
    let (start_pos, grid) = parse_file(file);
    let mut pipes = HashSet::new();
    pipes.insert(start_pos);
    let mut check = start_pos;
    loop {
        let mut any = false;
        for pos in adjacent_pipes(&grid, check) {
            if pipes.insert(pos) {
                check = pos;
                any = true;
            }
        }
        if !any {
            break;
        }
    }
    pipes.len() as u32 / 2
}

fn parse_file(file: &str) -> (Vec2<usize>, Vec<Vec<Option<Pipe>>>) {
    let file = read_file(file);
    let mut start_pos = None;
    let mut grid = file
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| {
                    if let 'S' = c {
                        start_pos = Some(Vec2::new(i, j));
                    }
                    Pipe::from_char(c)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let start_pos = start_pos.unwrap();
    let start_pipe = {
        let north = if start_pos.x == 0 {
            false
        } else {
            let npos = Vec2::new(start_pos.x - 1, start_pos.y);
            adjacent_pipes(&grid, npos).any(|p| p == start_pos)
        };
        let south = {
            let spos = Vec2::new(start_pos.x + 1, start_pos.y);
            adjacent_pipes(&grid, spos).any(|p| p == start_pos)
        };
        let east = {
            let epos = Vec2::new(start_pos.x, start_pos.y + 1);
            adjacent_pipes(&grid, epos).any(|p| p == start_pos)
        };
        let west = if start_pos.y == 0 {
            false
        } else {
            let wpos = Vec2::new(start_pos.x, start_pos.y - 1);
            adjacent_pipes(&grid, wpos).any(|p| p == start_pos)
        };
        if north && south {
            Pipe::NS
        } else if north && east {
            Pipe::NE
        } else if north && west {
            Pipe::NW
        } else if east && west {
            Pipe::EW
        } else if south && east {
            Pipe::SE
        } else if south && west {
            Pipe::SW
        } else {
            panic!("Yeet");
        }
    };
    grid[start_pos.x][start_pos.y] = Some(start_pipe);
    (start_pos, grid)
}

fn adjacent_pipes(
    grid: &[Vec<Option<Pipe>>],
    pos: Vec2<usize>,
) -> impl Iterator<Item = Vec2<usize>> + '_ {
    grid[pos.x][pos.y]
        .iter()
        .flat_map(move |p| p.adjacent_pipes(pos))
}

impl Pipe {
    fn south(grid: &[Vec<Option<Pipe>>], pos: Vec2<usize>) -> bool {
        matches!(grid[pos.x][pos.y], Some(Pipe::NS | Pipe::SE | Pipe::SW))
    }

    fn north(grid: &[Vec<Option<Pipe>>], pos: Vec2<usize>) -> bool {
        matches!(grid[pos.x][pos.y], Some(Pipe::NS | Pipe::NE | Pipe::NW))
    }

    fn adjacent_pipes(self, pos: Vec2<usize>) -> [Vec2<usize>; 2] {
        use Pipe::*;
        match self {
            NS => [Vec2::new(pos.x - 1, pos.y), Vec2::new(pos.x + 1, pos.y)],
            EW => [Vec2::new(pos.x, pos.y + 1), Vec2::new(pos.x, pos.y - 1)],
            NE => [Vec2::new(pos.x - 1, pos.y), Vec2::new(pos.x, pos.y + 1)],
            NW => [Vec2::new(pos.x - 1, pos.y), Vec2::new(pos.x, pos.y - 1)],
            SW => [Vec2::new(pos.x + 1, pos.y), Vec2::new(pos.x, pos.y - 1)],
            SE => [Vec2::new(pos.x + 1, pos.y), Vec2::new(pos.x, pos.y + 1)],
        }
    }

    fn from_char(c: char) -> Option<Pipe> {
        use Pipe::*;
        Some(match c {
            '|' => NS,
            '-' => EW,
            'L' => NE,
            'J' => NW,
            '7' => SW,
            'F' => SE,
            _ => {
                return None;
            }
        })
    }
}

#[derive(Copy, Clone, Debug)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}
