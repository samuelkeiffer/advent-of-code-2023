use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day12_test.txt"), 21);
    dbg!(part1("assets/day12.txt"));
    assert_eq!(part2("assets/day12_test.txt"), 525152);
    dbg!(part2("assets/day12.txt"));
}

fn part2(file: &str) -> u64 {
    let rows = parse_file2(file);
    rows.par_iter().map(|r| r.possible_states()).sum()
}

fn part1(file: &str) -> u64 {
    let rows = parse_file(file);
    rows.iter().map(|r| r.possible_states()).sum()
}

fn parse_file2(file: &str) -> Vec<Row> {
    let file = read_file(file);
    file.lines()
        .filter_map(|l| {
            l.split_once(' ').map(|(r, a)| {
                let init_row: Vec<State> = r.chars().filter_map(State::from_char).collect();
                let init_alt: Vec<usize> = a.split(',').filter_map(|n| n.parse().ok()).collect();
                let mut row = init_row.clone();
                let mut alt = init_alt.clone();
                for _ in 0..4 {
                    let mut x = init_row.clone();
                    row.push(State::Unkn);
                    row.append(&mut x);
                    let mut x = init_alt.clone();
                    alt.append(&mut x);
                }
                Row { row, alt }
            })
        })
        .collect()
}

fn parse_file(file: &str) -> Vec<Row> {
    let file = read_file(file);
    file.lines()
        .filter_map(|l| {
            l.split_once(' ').map(|(r, a)| Row {
                row: r.chars().filter_map(State::from_char).collect(),
                alt: a.split(',').filter_map(|n| n.parse().ok()).collect(),
            })
        })
        .collect()
}

fn possible_states(
    row: &[State],
    alt: &[usize],
    hash_map: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    let key = (row.len(), alt.len());
    if let Some(num_states) = hash_map.get(&key) {
        return *num_states;
    }

    if alt.is_empty() {
        return if row.iter().any(|e| matches!(e, State::Dmgd)) {
            hash_map.insert(key, 0);
            0
        } else {
            hash_map.insert(key, 1);
            1
        };
    }

    let min_length = alt.iter().sum::<usize>() + alt.len() - 1;
    if row.len() < min_length {
        hash_map.insert(key, 0);
        return 0;
    }

    if row.len() == min_length && alt.len() > 1 {
        return if row[..alt[0]].iter().any(|e| matches!(e, State::Oper))
            || matches!(row[alt[0]], State::Dmgd)
        {
            hash_map.insert(key, 0);
            0
        } else {
            let x = possible_states(&row[alt[0] + 1..], &alt[1..], hash_map);
            hash_map.insert(key, x);
            x
        };
    }

    if alt.len() == 1
        && row.len() == min_length
        && row.iter().all(|e| matches!(e, State::Dmgd | State::Unkn))
    {
        hash_map.insert(key, 1);
        return 1;
    }

    let mut num_states = 0;
    match row[0] {
        State::Oper => {
            num_states += possible_states(&row[1..], alt, hash_map);
        }
        State::Dmgd => {
            if row[..alt[0]]
                .iter()
                .all(|e| matches!(e, State::Dmgd | State::Unkn))
                && !matches!(row[alt[0]], State::Dmgd)
            {
                num_states += possible_states(&row[alt[0] + 1..], &alt[1..], hash_map);
            } else {
                hash_map.insert(key, 0);
                return 0;
            }
        }
        State::Unkn => {
            if row[..alt[0]]
                .iter()
                .all(|e| matches!(e, State::Dmgd | State::Unkn))
                && !matches!(row[alt[0]], State::Dmgd)
            {
                num_states += possible_states(&row[alt[0] + 1..], &alt[1..], hash_map);
            }
            num_states += possible_states(&row[1..], alt, hash_map);
        }
    }
    hash_map.insert(key, num_states);
    num_states
}

impl Row {
    fn possible_states(&self) -> u64 {
        let mut hash_map = HashMap::new();
        possible_states(&self.row, &self.alt, &mut hash_map)
    }
}

#[derive(Clone, Debug)]
struct Row {
    row: Vec<State>,
    alt: Vec<usize>,
}

impl State {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Oper),
            '#' => Some(Self::Dmgd),
            '?' => Some(Self::Unkn),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum State {
    Oper,
    Dmgd,
    Unkn,
}
