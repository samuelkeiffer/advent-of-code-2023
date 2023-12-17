use crate::*;

pub fn read_file(file: &str) -> String {
    fs::read_to_string(file).expect("Yeet")
}

impl Dir {
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

pub fn adjacent_indices_2d<T>(p: Vec2<usize>, grid: &[Vec<T>], diag: bool) -> Vec<Vec2<usize>> {
    adjacent_indices_2d_with_dir(p, grid, diag)
        .into_iter()
        .map(|(p, _d)| p)
        .collect::<Vec<_>>()
}

pub fn adjacent_indices_2d_with_dir<T>(
    p: Vec2<usize>,
    grid: &[Vec<T>],
    diag: bool,
) -> Vec<(Vec2<usize>, Dir)> {
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
