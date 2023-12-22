use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day3_test.txt"), 4361);
    dbg!(part1("assets/day3.txt"));
    assert_eq!(part2("assets/day3_test.txt"), 467835);
    dbg!(part2("assets/day3.txt"));
}

fn part2(file: &str) -> u32 {
    let schematic = parse_file(file);
    schematic
        .symbols
        .iter()
        .filter(|(_, sym)| **sym == '*')
        .map(|(pos, _)| {
            let mut adjacent_digits = Vec::new();
            for pos in adjacent_indices_2d(*pos, &schematic.raw, true, false) {
                if schematic.digits.contains_key(&pos) {
                    adjacent_digits.push(pos);
                }
            }
            let mut adjacent_numbers = HashSet::new();
            for digit in adjacent_digits {
                let mut number_pos = digit;
                loop {
                    if number_pos.y == 0 {
                        adjacent_numbers.insert(number_pos);
                        break;
                    }
                    let left_pos = Vec2::new(number_pos.x, number_pos.y - 1);
                    if schematic.digits.contains_key(&left_pos) {
                        number_pos = left_pos;
                    } else {
                        adjacent_numbers.insert(number_pos);
                        break;
                    }
                }
            }
            if adjacent_numbers.len() == 2 {
                adjacent_numbers
                    .iter()
                    .map(|pos| schematic.numbers.get(pos).unwrap())
                    .product()
            } else {
                0
            }
        })
        .sum()
}

fn part1(file: &str) -> u32 {
    let schematic = parse_file(file);
    schematic
        .numbers
        .iter()
        .filter_map(|(pos, number)| {
            let mut adjacent = false;
            for i in 0..=number.ilog10() {
                let pos = Vec2::new(pos.x, pos.y + i as usize);
                for pos in adjacent_indices_2d(pos, &schematic.raw, true, false) {
                    if schematic.symbols.contains_key(&pos) {
                        adjacent = true;
                    }
                }
            }
            adjacent.then_some(number)
        })
        .sum()
}

fn parse_file(file: &str) -> Schematic {
    let mut schematic = Schematic {
        raw: Vec::new(),
        symbols: HashMap::new(),
        digits: HashMap::new(),
        numbers: HashMap::new(),
    };

    let file = read_file(file);
    file.lines().enumerate().for_each(|(i, line)| {
        let mut row = Vec::new();
        let mut number: Option<(Vec2<usize>, String)> = None;
        for (j, ch) in line.chars().enumerate() {
            row.push(ch);
            let pos = Vec2::new(i, j);
            if ch.is_ascii_digit() {
                schematic.digits.insert(pos, ch);
                if let Some((_, ref mut num)) = &mut number {
                    num.push(ch);
                } else {
                    number = Some((pos, String::from(ch)))
                }
            } else {
                if ch != '.' {
                    schematic.symbols.insert(pos, ch);
                }
                if let Some((pos, num)) = number {
                    schematic.numbers.insert(pos, num.parse::<u32>().unwrap());
                    number = None;
                }
            }
        }
        if let Some((pos, num)) = number {
            schematic.numbers.insert(pos, num.parse::<u32>().unwrap());
        }
        schematic.raw.push(row);
    });

    schematic
}

#[derive(Debug, Clone)]
struct Schematic {
    raw: Vec<Vec<char>>,
    symbols: HashMap<Vec2<usize>, char>,
    digits: HashMap<Vec2<usize>, char>,
    numbers: HashMap<Vec2<usize>, u32>,
}
