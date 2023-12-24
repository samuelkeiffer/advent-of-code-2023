use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day23_test.txt"), 94);
    let handler = thread::Builder::new()
        .stack_size(1024 * 1024)
        .spawn(move || {
            dbg!(part1("assets/day23.txt"));
        })
        .unwrap();
    handler.join().unwrap();
    assert_eq!(part2("assets/day23_test.txt"), 154);
    dbg!(part2("assets/day23.txt"));
}

fn part2(file: &str) -> u64 {
    let forest = parse_file(file);
    let last_row = forest.len() - 1;
    let start_pos = forest[0]
        .iter()
        .enumerate()
        .find_map(|(i, t)| {
            if matches!(t, Tile::Path) {
                Some(Vec2::new(0, i))
            } else {
                None
            }
        })
        .unwrap();
    let end_pos = forest[last_row]
        .iter()
        .enumerate()
        .find_map(|(i, t)| {
            if matches!(t, Tile::Path) {
                Some(Vec2::new(last_row, i))
            } else {
                None
            }
        })
        .unwrap();

    let intersections = find_intersections(&forest, start_pos, end_pos);
    let path = vec![start_pos];

    longest_path2(path, 0, &intersections, end_pos)
}

fn longest_path2(
    path: Vec<Vec2<usize>>,
    cost: u64,
    intersections: &HashMap<Vec2<usize>, Vec<(Vec2<usize>, u64)>>,
    target: Vec2<usize>,
) -> u64 {
    let cur_pos = path.last().unwrap();
    if *cur_pos == target {
        return cost;
    }
    intersections
        .get(cur_pos)
        .unwrap()
        .iter()
        .filter(|(dest, _)| !path.contains(dest))
        .map(|(dest, length)| {
            let mut path = path.clone();
            path.push(*dest);
            longest_path2(path, cost + length, intersections, target)
        })
        .max()
        .unwrap_or(0)
}

fn find_intersections(
    forest: &[Vec<Tile>],
    start_pos: Vec2<usize>,
    end_pos: Vec2<usize>,
) -> HashMap<Vec2<usize>, Vec<(Vec2<usize>, u64)>> {
    let intersections = forest
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, tile)| {
                if let Tile::Forest = tile {
                    None
                } else {
                    let pos = Vec2::new(i, j);
                    let adj_pos = adjacent_indices_2d(pos, forest, false, false);
                    let num_adj = adj_pos
                        .iter()
                        .filter(|adj| forest[adj.x][adj.y] != Tile::Forest)
                        .count();
                    if num_adj > 2 || pos == start_pos || pos == end_pos {
                        Some((pos, adj_pos))
                    } else {
                        None
                    }
                }
            })
        })
        .collect::<Vec<_>>();

    intersections
        .iter()
        .map(|(pos, adjs)| {
            let mut connections = Vec::new();
            for adj in adjs {
                let mut path = vec![*adj];
                loop {
                    let last = path.last().unwrap();
                    if intersections.iter().any(|(pos, _)| pos == last) {
                        connections.push((*last, path.len() as u64));
                        break;
                    } else {
                        let next = adjacent_indices_2d(*last, forest, false, false);
                        if let Some(nxt) = next.into_iter().find(|nxt| {
                            let visited = path.contains(nxt);
                            let clear = forest[nxt.x][nxt.y] != Tile::Forest;
                            clear && !visited && nxt != pos
                        }) {
                            path.push(nxt);
                        } else {
                            break;
                        }
                    }
                }
            }
            (*pos, connections)
        })
        .collect::<HashMap<_, _>>()
}

fn part1(file: &str) -> u64 {
    let forest = parse_file(file);
    let last_row = forest.len() - 1;
    let start_pos = forest[0]
        .iter()
        .enumerate()
        .find_map(|(i, t)| {
            if matches!(t, Tile::Path) {
                Some(Vec2::new(0, i))
            } else {
                None
            }
        })
        .unwrap();
    let end_pos = forest[last_row]
        .iter()
        .enumerate()
        .find_map(|(i, t)| {
            if matches!(t, Tile::Path) {
                Some(Vec2::new(last_row, i))
            } else {
                None
            }
        })
        .unwrap();

    let path = vec![start_pos];
    longest_path(path, &forest, end_pos, true)
}

fn longest_path(
    mut path: Vec<Vec2<usize>>,
    forest: &[Vec<Tile>],
    target: Vec2<usize>,
    one_way: bool,
) -> u64 {
    let cur_pos = path.last().unwrap();
    if *cur_pos == target {
        return path.len() as u64 - 1;
    }
    let adj_pos = adjacent_indices_2d_with_dir(*cur_pos, forest, false, false);
    let adj_pos = adj_pos
        .into_iter()
        .filter(|(adj, travel_dir)| {
            let dest_clear = forest[adj.x][adj.y] != Tile::Forest;
            let not_up = if let Tile::Slope(dir) = forest[cur_pos.x][cur_pos.y] {
                *travel_dir == dir || !one_way
            } else {
                true
            };
            let unvisited = !path.contains(adj);
            dest_clear && not_up && unvisited
        })
        .collect::<Vec<_>>();

    if adj_pos.is_empty() {
        0
    } else if adj_pos.len() == 1 {
        path.push(adj_pos[0].0);
        longest_path(path, forest, target, one_way)
    } else {
        adj_pos
            .into_iter()
            .map(|(adj, _)| {
                let mut path = path.clone();
                path.push(adj);
                longest_path(path, forest, target, one_way)
            })
            .max()
            .unwrap_or(0)
    }
}

fn parse_file(file: &str) -> Vec<Vec<Tile>> {
    let file = read_file(file);
    file.lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Tile::Forest,
                    '.' => Tile::Path,
                    '^' => Tile::Slope(Dir::Up),
                    'v' => Tile::Slope(Dir::Down),
                    '<' => Tile::Slope(Dir::Left),
                    '>' => Tile::Slope(Dir::Right),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Forest,
    Path,
    Slope(Dir),
}
