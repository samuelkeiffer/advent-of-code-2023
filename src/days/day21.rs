use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day21_test.txt", 6), 16);
    dbg!(part1("assets/day21.txt", 64));
    // assert_eq!(part2("assets/day21_test.txt", 10), 50);
    // assert_eq!(part2("assets/day21_test.txt", 50), 1594);
    // assert_eq!(part2("assets/day21_test.txt", 100), 6536);
    // assert_eq!(part2("assets/day21_test.txt", 500), 167004);
    // assert_eq!(part2("assets/day21_test.txt", 1000), 668697);
    // assert_eq!(part2("assets/day21_test.txt", 5000), 16733044);
    dbg!(part2("assets/day21.txt", 26501365));
}

fn part2(file: &str, num_steps: i64) -> u128 {
    let (start_pos, grid) = parse_file(file);
    let grid = grid
        .into_iter()
        .map(|r| {
            r.into_iter()
                .map(|e| if e { Some(false) } else { None })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut garden = grid.clone();
    garden[start_pos.x][start_pos.y] = Some(true);
    let mut gardens = HashMap::new();
    gardens.insert((0, 0), garden);

    let possible = |gardens: &HashMap<(i32, i32), Vec<Vec<Option<bool>>>>| {
        gardens
            .values()
            .map(|g| {
                g.iter()
                    .map(|r| r.iter().filter_map(|x| *x).filter(|x| *x).count())
                    .sum::<usize>() as u128
            })
            .sum::<u128>()
    };

    let mut quadratic = HashMap::new();

    for i in 0..num_steps {
        let repeats = (i - 65) / 131;
        if (i - 65) % 131 == 0 {
            dbg!(i);
            dbg!(repeats);
            dbg!(possible(&gardens));
            quadratic.insert(repeats as u128, possible(&gardens));
        }
        hyper_step(&mut gardens, &grid);
        if quadratic.len() == 3 {
            break;
        }
    }

    let c = quadratic.get(&0).unwrap();
    let (a, b) = {
        let p2 = quadratic.get(&2).unwrap();
        let p1 = quadratic.get(&1).unwrap();
        // p2 = 4 * a + 2 * b + c;
        // p1 = a + b + c;
        let a = (p2 - 2 * p1 + c) / 2;
        let b = p1 - a - c;
        (a, b)
    };
    let x = (num_steps as u128 - 65) / 131;
    a * x.pow(2) + b * x + c
}

fn hyper_step(
    gardens: &mut HashMap<(i32, i32), Vec<Vec<Option<bool>>>>,
    grid: &[Vec<Option<bool>>],
) {
    let mut new_gardens = HashMap::new();
    for ((x, y), garden) in gardens.iter_mut() {
        let mut off = Vec::new();
        let mut on = Vec::new();
        for (i, row) in garden.iter().enumerate() {
            for (j, plot) in row
                .iter()
                .enumerate()
                .filter_map(|(j, x)| x.map(|x| (j, x)))
            {
                if plot {
                    let pos = Vec2::new(i, j);
                    off.push(pos);
                    let for_check = adjacent_indices_2d(pos, garden, false, false);
                    for (adj, dir) in adjacent_indices_2d_with_dir(pos, garden, false, true) {
                        if for_check.contains(&adj) {
                            on.push(adj);
                        } else {
                            let new_offset = Vec2::new(*x, *y) + dir.to_vec();
                            let new_garden = new_gardens
                                .entry((new_offset.x, new_offset.y))
                                .or_insert(grid.to_vec());
                            if let Some(Some(plot)) =
                                new_garden.get_mut(adj.x).and_then(|row| row.get_mut(adj.y))
                            {
                                *plot = true;
                            }
                        }
                    }
                }
            }
        }
        for pos in off {
            garden[pos.x][pos.y] = Some(false);
        }
        for pos in on {
            if let Some(Some(plot)) = garden.get_mut(pos.x).and_then(|row| row.get_mut(pos.y)) {
                *plot = true;
            }
        }
    }
    for (offset, new_garden) in new_gardens {
        let garden = gardens.entry(offset).or_insert(new_garden.clone());
        for (i, row) in new_garden.iter().enumerate() {
            for (j, plot) in row.iter().enumerate() {
                if matches!(plot, Some(true)) {
                    garden[i][j] = Some(true);
                }
            }
        }
    }
}

fn part1(file: &str, num_steps: u32) -> u64 {
    let (start_pos, grid) = parse_file(file);
    let mut garden = grid
        .into_iter()
        .map(|r| {
            r.into_iter()
                .map(|e| if e { Some(false) } else { None })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    garden[start_pos.x][start_pos.y] = Some(true);
    for _ in 0..num_steps {
        step(&mut garden);
        // dbg_grid(&garden, |x| match x {
        //     Some(true) => 'O',
        //     Some(false) => '.',
        //     None => '#',
        // });
    }
    garden
        .iter()
        .map(|r| r.iter().filter_map(|x| *x).filter(|x| *x).count())
        .sum::<usize>() as u64
}

fn step(garden: &mut [Vec<Option<bool>>]) {
    let mut off = Vec::new();
    let mut on = Vec::new();
    for (i, row) in garden.iter().enumerate() {
        for (j, plot) in row
            .iter()
            .enumerate()
            .filter_map(|(j, x)| x.map(|x| (j, x)))
        {
            if plot {
                let pos = Vec2::new(i, j);
                off.push(pos);
                for adj in adjacent_indices_2d(pos, garden, false, false) {
                    on.push(adj);
                }
            }
        }
    }
    for pos in off {
        garden[pos.x][pos.y] = Some(false);
    }
    for pos in on {
        if let Some(Some(plot)) = garden.get_mut(pos.x).and_then(|row| row.get_mut(pos.y)) {
            *plot = true;
        }
    }
}

fn parse_file(file: &str) -> (Vec2<usize>, Vec<Vec<bool>>) {
    let file = read_file(file);
    let grid = read_bool_grid(&file, |c| matches!(c, '.' | 'S'));
    let start_pos = file
        .lines()
        .enumerate()
        .find_map(|(i, r)| {
            r.chars().enumerate().find_map(|(j, c)| {
                if let 'S' = c {
                    Some(Vec2::new(i, j))
                } else {
                    None
                }
            })
        })
        .unwrap();
    (start_pos, grid)
}
