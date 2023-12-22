use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day17_test.txt"), 102);
    dbg!(part1("assets/day17.txt"));
    assert_eq!(part2("assets/day17_test.txt"), 94);
    assert_eq!(part2("assets/day17_test2.txt"), 14);
    assert_eq!(part2("assets/day17_test3.txt"), 71);
    assert_eq!(part2("assets/day17_test4.txt"), 14);
    dbg!(part2("assets/day17.txt"));
}

fn part2(file: &str) -> u64 {
    let grid = parse_file(file);
    let end = Vec2::new(grid.len() - 1, grid[0].len() - 1);
    let adjacent_nodes = |(p, d, i): (Vec2<usize>, Dir, usize)| {
        let mut vec = adjacent_indices_2d_with_dir(p, &grid, false, false)
            .into_iter()
            .map(|(p, e)| ((p, e, if d == e { i + 1 } else { 1 }), grid[p.x][p.y]))
            .collect::<Vec<_>>();
        vec.retain(|((_, e, j), _)| {
            let dist_check = {
                if d != *e {
                    i > 3
                } else {
                    *j <= 10
                }
            };
            dist_check && !d.opposite(e)
        });
        vec
    };
    let (_path1, cost1) = dijkstra(
        &(Vec2::new(0, 0), Dir::Right, 0),
        |p| adjacent_nodes(*p),
        |p| p.0 == end && p.2 > 3,
    )
    .unwrap();
    let (_path2, cost2) = dijkstra(
        &(Vec2::new(0, 0), Dir::Down, 0),
        |p| adjacent_nodes(*p),
        |p| p.0 == end && p.2 > 3,
    )
    .unwrap();
    cost1.min(cost2) as u64
}

fn part1(file: &str) -> u64 {
    let grid = parse_file(file);
    let end = Vec2::new(grid.len() - 1, grid[0].len() - 1);
    let adjacent_nodes = |(p, d, i): (Vec2<usize>, Dir, usize)| {
        let mut vec = adjacent_indices_2d_with_dir(p, &grid, false, false)
            .into_iter()
            .map(|(p, e)| ((p, e, if d == e { i + 1 } else { 0 }), grid[p.x][p.y]))
            .collect::<Vec<_>>();
        vec.retain(|((_, e, i), _)| *i != 3 && !d.opposite(e));
        vec
    };
    let (_path, cost) = dijkstra(
        &(Vec2::new(0, 0), Dir::Down, 0),
        |p| adjacent_nodes(*p),
        |p| p.0 == end,
    )
    .unwrap();
    cost as u64
}

fn parse_file(file: &str) -> Vec<Vec<u32>> {
    let file = read_file(file);
    file.lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
