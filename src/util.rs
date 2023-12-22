use crate::*;

pub fn read_file(file: &str) -> String {
    fs::read_to_string(file).expect("Yeet")
}

pub fn read_bool_grid<F: Fn(char) -> bool>(grid: &str, condition: F) -> Vec<Vec<bool>> {
    grid.lines()
        .map(|l| l.chars().map(&condition).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn dbg_grid<T, F: Fn(&T) -> char>(grid: &[Vec<T>], conv: F) {
    for row in grid.iter() {
        println!("{}", row.iter().map(&conv).collect::<String>());
    }
    println!(" ");
}

impl Dir {
    pub fn to_vec(&self) -> Vec2<i32> {
        match self {
            Dir::Up => Vec2::new(-1, 0),
            Dir::Down => Vec2::new(1, 0),
            Dir::Left => Vec2::new(0, -1),
            Dir::Right => Vec2::new(0, 1),
        }
    }

    pub fn next_pos(&self, pos: Vec2<usize>) -> Option<Vec2<usize>> {
        match self {
            Dir::Up => {
                if pos.x == 0 {
                    None
                } else {
                    Some(Vec2::new(pos.x - 1, pos.y))
                }
            }
            Dir::Down => Some(Vec2::new(pos.x + 1, pos.y)),
            Dir::Left => {
                if pos.y == 0 {
                    None
                } else {
                    Some(Vec2::new(pos.x, pos.y - 1))
                }
            }
            Dir::Right => Some(Vec2::new(pos.x, pos.y + 1)),
        }
    }

    pub fn opposite(&self, other: &Self) -> bool {
        match self {
            Dir::Up => matches!(other, Dir::Down),
            Dir::Down => matches!(other, Dir::Up),
            Dir::Left => matches!(other, Dir::Right),
            Dir::Right => matches!(other, Dir::Left),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn mirrored_indices<T: Copy>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    grid.iter()
        .map(|r| r.iter().copied().rev().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn inverted_indices<T: Copy>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    (0..grid[0].len())
        .map(|i| grid.iter().map(|g| g[i]).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn numeric_digits_from_line(text: &str, line: usize) -> String {
    text.lines()
        .nth(line)
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect()
}

pub fn adjacent_pos_i32(p: &Vec2<i32>) -> Vec<Vec2<i32>> {
    vec![
        Vec2::new(p.x - 1, p.y - 1),
        Vec2::new(p.x - 1, p.y),
        Vec2::new(p.x - 1, p.y + 1),
        Vec2::new(p.x, p.y - 1),
        Vec2::new(p.x, p.y + 1),
        Vec2::new(p.x + 1, p.y - 1),
        Vec2::new(p.x + 1, p.y),
        Vec2::new(p.x + 1, p.y + 1),
    ]
}

pub fn adjacent_indices_2d<T>(
    p: Vec2<usize>,
    grid: &[Vec<T>],
    diag: bool,
    wrap: bool,
) -> Vec<Vec2<usize>> {
    adjacent_indices_2d_with_dir(p, grid, diag, wrap)
        .into_iter()
        .map(|(p, _d)| p)
        .collect::<Vec<_>>()
}

pub fn adjacent_indices_2d_with_dir<T>(
    p: Vec2<usize>,
    grid: &[Vec<T>],
    diag: bool,
    wrap: bool,
) -> Vec<(Vec2<usize>, Dir)> {
    if wrap {
        let modulus = |x, add| {
            if x {
                let val = p.x + grid.len();
                let val = if add { val + 1 } else { val - 1 };
                val % grid.len()
            } else {
                let val = p.y + grid[0].len();
                let val = if add { val + 1 } else { val - 1 };
                val % grid[0].len()
            }
        };
        if diag {
            vec![
                (
                    Vec2::new(modulus(true, false), modulus(false, false)),
                    Dir::Up,
                ),
                (Vec2::new(modulus(true, false), p.y), Dir::Up),
                (
                    Vec2::new(modulus(true, false), modulus(false, true)),
                    Dir::Up,
                ),
                (Vec2::new(p.x, modulus(false, false)), Dir::Left),
                (Vec2::new(p.x, modulus(false, true)), Dir::Right),
                (
                    Vec2::new(modulus(true, true), modulus(false, false)),
                    Dir::Down,
                ),
                (Vec2::new(modulus(true, true), p.y), Dir::Down),
                (
                    Vec2::new(modulus(true, true), modulus(false, true)),
                    Dir::Down,
                ),
            ]
        } else {
            vec![
                (Vec2::new(modulus(true, false), p.y), Dir::Up),
                (Vec2::new(p.x, modulus(false, false)), Dir::Left),
                (Vec2::new(p.x, modulus(false, true)), Dir::Right),
                (Vec2::new(modulus(true, true), p.y), Dir::Down),
            ]
        }
    } else {
        let mut candidates = if p.x == 0 && p.y == 0 {
            vec![
                (Vec2::new(p.x, p.y + 1), Dir::Right),
                (Vec2::new(p.x + 1, p.y), Dir::Down),
                (Vec2::new(p.x + 1, p.y + 1), Dir::Down),
            ]
        } else if p.x == 0 {
            vec![
                (Vec2::new(p.x, p.y - 1), Dir::Left),
                (Vec2::new(p.x, p.y + 1), Dir::Right),
                (Vec2::new(p.x + 1, p.y - 1), Dir::Down),
                (Vec2::new(p.x + 1, p.y), Dir::Down),
                (Vec2::new(p.x + 1, p.y + 1), Dir::Down),
            ]
        } else if p.y == 0 {
            vec![
                (Vec2::new(p.x - 1, p.y), Dir::Up),
                (Vec2::new(p.x - 1, p.y + 1), Dir::Up),
                (Vec2::new(p.x, p.y + 1), Dir::Right),
                (Vec2::new(p.x + 1, p.y), Dir::Down),
                (Vec2::new(p.x + 1, p.y + 1), Dir::Down),
            ]
        } else {
            vec![
                (Vec2::new(p.x - 1, p.y - 1), Dir::Up),
                (Vec2::new(p.x - 1, p.y), Dir::Up),
                (Vec2::new(p.x - 1, p.y + 1), Dir::Up),
                (Vec2::new(p.x, p.y - 1), Dir::Left),
                (Vec2::new(p.x, p.y + 1), Dir::Right),
                (Vec2::new(p.x + 1, p.y - 1), Dir::Down),
                (Vec2::new(p.x + 1, p.y), Dir::Down),
                (Vec2::new(p.x + 1, p.y + 1), Dir::Down),
            ]
        };

        candidates.retain(|(p, _)| grid.get(p.x).and_then(|r| r.get(p.y)).is_some());
        candidates.retain(|(q, _)| {
            let p = p.as_::<i64>();
            let q = q.as_::<i64>();
            let mnhtn_dist = (p.x - q.x).abs() + (p.y - q.y).abs();
            diag || mnhtn_dist == 1
        });
        candidates
    }
}
