use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day22_test.txt"), 5);
    dbg!(part1("assets/day22.txt"));
    assert_eq!(part2("assets/day22_test.txt"), 7);
    dbg!(part2("assets/day22.txt"));
}

fn part2(file: &str) -> u64 {
    let bricks = parse_file(file);
    let mut vek_bricks = bricks
        .iter()
        .map(|b| Aabb {
            min: b.min,
            max: b.max + Vec3::new(1, 1, 1),
        })
        .collect::<Vec<_>>();
    fall(&mut vek_bricks);
    (0..vek_bricks.len())
        .map(|i| {
            let mut bricks = vek_bricks.clone();
            bricks.swap_remove(i);
            num_fall(bricks)
        })
        .sum::<u64>()
}

fn num_fall(mut bricks: Vec<Aabb<u64>>) -> u64 {
    let mut fell = HashSet::new();
    loop {
        let mut moved = false;
        for i in 0..bricks.len() {
            let brick = bricks[i];
            if brick.min.z == 1 {
                continue;
            }
            let fall = Aabb {
                min: brick.min - Vec3::new(0, 0, 1),
                max: brick.max - Vec3::new(0, 0, 1),
            };
            if bricks
                .iter()
                .enumerate()
                .any(|(j, brick2)| i != j && fall.collides_with_aabb(*brick2))
            {
                continue;
            } else {
                fell.insert(i);
                moved = true;
                bricks[i] = fall;
            }
        }
        if !moved {
            return fell.len() as u64;
        }
    }
}

fn part1(file: &str) -> u64 {
    let bricks = parse_file(file);
    let mut vek_bricks = bricks
        .iter()
        .map(|b| Aabb {
            min: b.min,
            max: b.max + Vec3::new(1, 1, 1),
        })
        .collect::<Vec<_>>();
    fall(&mut vek_bricks);
    could_remove(&vek_bricks)
}

fn could_remove(bricks: &[Aabb<u64>]) -> u64 {
    let mut count = 0;
    for i in 0..bricks.len() {
        for (j, brick) in bricks.iter().enumerate().filter(|(j, _)| i != *j) {
            if brick.min.z == 1 {
                continue;
            }
            let fall = Aabb {
                min: brick.min - Vec3::new(0, 0, 1),
                max: brick.max - Vec3::new(0, 0, 1),
            };
            if bricks
                .iter()
                .enumerate()
                .filter(|(k, _)| *k != i && *k != j)
                .any(|(_, brick2)| fall.collides_with_aabb(*brick2))
            {
                continue;
            } else {
                count += 1;
                break;
            }
        }
    }
    bricks.len() as u64 - count
}

fn fall(bricks: &mut [Aabb<u64>]) {
    loop {
        let mut moved = false;
        for i in 0..bricks.len() {
            let brick = bricks[i];
            if brick.min.z == 1 {
                continue;
            }
            let fall = Aabb {
                min: brick.min - Vec3::new(0, 0, 1),
                max: brick.max - Vec3::new(0, 0, 1),
            };
            if bricks
                .iter()
                .enumerate()
                .any(|(j, brick2)| i != j && fall.collides_with_aabb(*brick2))
            {
                continue;
            } else {
                moved = true;
                bricks[i] = fall;
            }
        }
        if !moved {
            return;
        }
    }
}

fn parse_file(file: &str) -> Vec<Aabb<u64>> {
    let file = read_file(file);
    file.lines()
        .map(|l| {
            let (x1, y1, z1, x2, y2, z2): (u64, u64, u64, u64, u64, u64);
            scan!(l.bytes() => "{},{},{}~{},{},{}", x1, y1, z1, x2, y2, z2);
            let p1 = Vec3::new(x1, y1, z1);
            let p2 = Vec3::new(x2, y2, z2);
            let aabb = Aabb { min: p1, max: p2 };
            aabb.made_valid()
        })
        .collect::<Vec<_>>()
}
