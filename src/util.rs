use crate::*;

pub fn read_file(file: &str) -> String {
    fs::read_to_string(file).expect("Yeet")
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
