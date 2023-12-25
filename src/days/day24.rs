use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day24_test.txt", (7, 27)), 2);
    dbg!(part1(
        "assets/day24.txt",
        (200000000000000, 400000000000000)
    ));
    assert_eq!(part2("assets/day24_test.txt"), 47);
    dbg!(part2("assets/day24.txt"));
}

fn part2(file: &str) -> u64 {
    #[rustfmt::skip]
    // pr + vr * t1 = p1 + v1 * t1
    // prx + vrx * t1 = p1x + v1x * t1
    // pry + vry * t1 = p1y + v1y * t1
    // t1 = (pry - p1y) / (v1y - vry)
    // (prx - p1x) / (v1x - vrx) = (pry - p1y) / (v1y - vry)
    // (prx - p1x) * (v1y - vry) = (pry - p1y) * (v1x - vrx)
    // prx * v1y - prx * vry - p1x * v1y + p1x * vry = pry * v1x - pry * vrx - p1y * v1x + p1y * vrx
    // pry * vrx - prx * vry = -v1y * prx + v1x * pry + p1y * vrx - p1x * vry - p1y * v1x + p1x * v1y
    // -v1y * prx + v1x * pry + p1y * vrx - p1x * vry - p1y * v1x + p1x * v1y = -v2y * prx + v2x * pry + p2y * vrx - p2x * vry - p2y * v2x + p2x * v2y
    // (v2y - v1y) * prx + (v1x - v2x) * pry + (p1y - p2y) * vrx + (p2x - p1x) * vry = p1y * v1x - p1x * v1y - p2y * v2x + p2x * v2y
    let h = parse_file(file);
    let mut prxs = HashMap::new();
    let mut prys = HashMap::new();
    let mut przs = HashMap::new();
    for i in 4..h.len() {
        let check = ((i - 4)..(i + 1)).map(|i| h[i]).collect::<Vec<_>>();
        let ref_pos = check[0].pos;
        let check = check
            .into_iter()
            .map(|h| Hail {
                pos: h.pos - ref_pos,
                vel: h.vel,
            })
            .collect::<Vec<_>>();
        let mut matrix = (1..5)
            .map(|i| {
                let h1 = check[0];
                let h2 = check[i];
                let row = vec![
                    h2.vel.y - h1.vel.y,
                    h1.vel.x - h2.vel.x,
                    h1.pos.y - h2.pos.y,
                    h2.pos.x - h1.pos.x,
                    h1.pos.y * h1.vel.x - h1.pos.x * h1.vel.y - h2.pos.y * h2.vel.x
                        + h2.pos.x * h2.vel.y,
                ];
                row
            })
            .collect::<Vec<_>>();
        gauss_jordan_elimination_generic::<f64>(&mut matrix);
        let prx = (matrix[0][4].round() + ref_pos.x) as u64;
        let pry = (matrix[1][4].round() + ref_pos.y) as u64;
        let mut matrix = (1..5)
            .map(|i| {
                let h1 = check[0];
                let h2 = check[i];
                let row = vec![
                    h2.vel.z - h1.vel.z,
                    h1.vel.x - h2.vel.x,
                    h1.pos.z - h2.pos.z,
                    h2.pos.x - h1.pos.x,
                    h1.pos.z * h1.vel.x - h1.pos.x * h1.vel.z - h2.pos.z * h2.vel.x
                        + h2.pos.x * h2.vel.z,
                ];
                row
            })
            .collect::<Vec<_>>();
        gauss_jordan_elimination_generic::<f64>(&mut matrix);
        let prz = (matrix[1][4].round() + ref_pos.z) as u64;
        let entry_x = prxs.entry(prx).or_insert(0);
        *entry_x += 1;
        let entry_y = prys.entry(pry).or_insert(0);
        *entry_y += 1;
        let entry_z = przs.entry(prz).or_insert(0);
        *entry_z += 1;
    }
    let prx = prxs
        .into_iter()
        .fold((0, 0), |a, b| if b.1 > a.1 { b } else { a })
        .0;
    let pry = prys
        .into_iter()
        .fold((0, 0), |a, b| if b.1 > a.1 { b } else { a })
        .0;
    let prz = przs
        .into_iter()
        .fold((0, 0), |a, b| if b.1 > a.1 { b } else { a })
        .0;
    prx + pry + prz
}

fn part1(file: &str, bounds: (u64, u64)) -> u64 {
    let hailstorm = parse_file(file);
    hailstorm
        .iter()
        .enumerate()
        .flat_map(|(i, hail)| {
            hailstorm
                .iter()
                .skip(i + 1)
                .map(move |hail2| (*hail, hail2))
        })
        .map(|(h1, h2)| {
            let (t1, t2) = h1.collision_times_2d(h2);
            ((h1, t1), (h2, t2))
        })
        .filter(|((h1, t1), (_h2, t2))| {
            let int = h1.pos + h1.vel * *t1;
            let future = *t1 >= 0.0 && *t2 >= 0.0;
            let bounded = {
                let lo = bounds.0 as f64;
                let hi = bounds.1 as f64;
                (lo..hi).contains(&int.x) && (lo..hi).contains(&int.y)
            };
            future && bounded
        })
        .count() as u64
}

fn parse_file(file: &str) -> Vec<Hail> {
    let file = read_file(file);
    file.lines()
        .map(|l| {
            let (px, py, pz, vx, vy, vz): (f64, f64, f64, f64, f64, f64);
            scan!(l.bytes() => "{}, {}, {} @ {}, {}, {}", px, py, pz, vx, vy, vz);
            Hail {
                pos: Vec3::new(px, py, pz),
                vel: Vec3::new(vx, vy, vz),
            }
        })
        .collect::<Vec<_>>()
}

impl Hail {
    fn collision_times_2d(&self, other: &Self) -> (f64, f64) {
        // x1 + vx1 * t1 = x2 + vx2 * t2
        // y1 + vy1 * t1 = y2 + vy2 * t2
        // t1 = (y2 + vy2 * t2 - y1) / vy1
        // (y2 + vy2 * t2 - y1) / vy1 = (x2 + vx2 * t2 - x1) / vx1
        // (y2 - y1) / vy1 + (vy2 / vy1) * t2 = (x2 - x1) / vx1 + (vx2 / vx1) * t2
        // (y2 - y1) / vy1 - (x2 - x1) / vx1 = (vx2 / vx1 - vy2 / vy1) * t2
        // t2 = ((y2 - y1) / vy1 - (x2 - x1) / vx1) / (vx2 / vx1 - vy2 / vy1)
        if self.vel.x.abs() > f64::EPSILON && self.vel.y.abs() > f64::EPSILON {
            let (h1, h2) = (self, other);
            let t2 = ((h2.pos.y - h1.pos.y) / h1.vel.y - (h2.pos.x - h1.pos.x) / h1.vel.x)
                / (h2.vel.x / h1.vel.x - h2.vel.y / h1.vel.y);
            let t1 = (h2.pos.y + h2.vel.y * t2 - h1.pos.y) / h1.vel.y;
            (t1, t2)
        } else {
            let (h2, h1) = (self, other);
            let t2 = ((h2.pos.y - h1.pos.y) / h1.vel.y - (h2.pos.x - h1.pos.x) / h1.vel.x)
                / (h2.vel.x / h1.vel.x - h2.vel.y / h1.vel.y);
            let t1 = (h2.pos.y + h2.vel.y * t2 - h1.pos.y) / h1.vel.y;
            (t2, t1)
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Hail {
    pos: Vec3<f64>,
    vel: Vec3<f64>,
}
