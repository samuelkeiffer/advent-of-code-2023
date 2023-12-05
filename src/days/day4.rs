use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day4_test.txt"), 13);
    dbg!(part1("assets/day4.txt"));
    assert_eq!(part2("assets/day4_test.txt"), 30);
    dbg!(part2("assets/day4.txt"));
}

fn part2(file: &str) -> u32 {
    let cards = parse_file(file);
    let mut copies = cards
        .iter()
        .enumerate()
        .map(|(i, _)| (i, 1))
        .collect::<BTreeMap<_, _>>();
    for (i, card) in cards.iter().enumerate() {
        let matches = card.wins();
        let num = *copies.get(&i).unwrap();
        for j in 0..matches {
            if let Some(next) = copies.get_mut(&(i + j + 1)) {
                *next += num
            }
        }
    }
    copies.values().sum()
}

fn part1(file: &str) -> u32 {
    let cards = parse_file(file);
    cards
        .iter()
        .filter_map(|card| {
            let matches = card.wins();
            (matches > 0).then_some(2_u32.pow((matches as u32).saturating_sub(1)))
        })
        .sum()
}

fn parse_file(file: &str) -> Vec<Card> {
    let file = read_file(file);
    file.lines()
        .map(|line| {
            let card = line.split_once(':').unwrap().1;
            let (win, num) = card.split_once('|').unwrap();
            Card {
                win: win
                    .split(' ')
                    .filter_map(|w| w.parse::<u32>().ok())
                    .collect(),
                num: num
                    .split(' ')
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect(),
            }
        })
        .collect()
}

#[derive(Clone, Debug)]
struct Card {
    win: Vec<u32>,
    num: Vec<u32>,
}

impl Card {
    fn wins(&self) -> usize {
        self.win.iter().filter(|w| self.num.contains(w)).count()
    }
}
