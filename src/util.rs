use crate::*;

pub fn read_file(file: &str) -> String {
    fs::read_to_string(file).expect("Yeet")
}

impl Dir {
    pub fn next_pos(&self, pos: Vec2<usize>) -> Option<Vec2<usize>> {
        match self {
            Self::Up => {
                if pos.x == 0 {
                    None
                } else {
                    Some(Vec2::new(pos.x - 1, pos.y))
                }
            }
            Self::Down => Some(Vec2::new(pos.x + 1, pos.y)),
            Self::Left => {
                if pos.y == 0 {
                    None
                } else {
                    Some(Vec2::new(pos.x, pos.y - 1))
                }
            }
            Self::Right => Some(Vec2::new(pos.x, pos.y + 1)),
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

pub fn adjacent_indices_2d(p: Vec2<usize>) -> Vec<Vec2<usize>> {
    if p.x == 0 && p.y == 0 {
        vec![
            Vec2::new(p.x, p.y + 1),
            Vec2::new(p.x + 1, p.y),
            Vec2::new(p.x + 1, p.y + 1),
        ]
    } else if p.x == 0 {
        vec![
            Vec2::new(p.x, p.y - 1),
            Vec2::new(p.x, p.y + 1),
            Vec2::new(p.x + 1, p.y - 1),
            Vec2::new(p.x + 1, p.y),
            Vec2::new(p.x + 1, p.y + 1),
        ]
    } else if p.y == 0 {
        vec![
            Vec2::new(p.x - 1, p.y),
            Vec2::new(p.x - 1, p.y + 1),
            Vec2::new(p.x, p.y + 1),
            Vec2::new(p.x + 1, p.y),
            Vec2::new(p.x + 1, p.y + 1),
        ]
    } else {
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
}
