use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day7_test.txt"), 6440);
    dbg!(part1("assets/day7.txt"));
    assert_eq!(part2("assets/day7_test.txt"), 5905);
    dbg!(part2("assets/day7.txt"));
}

fn part2(file: &str) -> u32 {
    let hands = parse_file(file, true);
    hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

fn part1(file: &str) -> u32 {
    let hands = parse_file(file, false);
    hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

fn parse_file(file: &str, joker: bool) -> Vec<Hand> {
    let file = read_file(file);
    let mut hands = Vec::new();
    for (cards, bid) in file.lines().filter_map(|l| l.split_once(' ')) {
        let mut hand = Hand {
            cards: HashMap::new(),
            order: [0; 5],
            bid: bid.parse::<u32>().unwrap(),
        };
        for (i, card) in cards.chars().enumerate() {
            let card = parse_card(card, joker);
            hand.order[i] = card;
            let entry = hand.cards.entry(card).or_insert(0);
            *entry += 1;
        }
        if joker {
            let num = hand.cards.get(&0).copied().unwrap_or(0);
            let mut highest = (0, 0);
            for (card, amount) in hand.cards.iter() {
                if *amount > highest.1 && *card != 0 {
                    highest = (*card, *amount)
                }
            }
            if highest.0 != 0 {
                if let Some(amount) = hand.cards.get_mut(&highest.0) {
                    *amount += num
                }
                hand.cards.remove(&0);
            }
        }
        hands.push(hand);
    }
    hands
}

enum HandKind {
    Five = 0,
    Four = 1,
    Full = 2,
    Three = 3,
    Pair = 4,
    Two = 5,
    High = 6,
}

impl Hand {
    fn kind(&self) -> HandKind {
        let mut twos = 0;
        let mut threes = 0;
        for v in self.cards.values() {
            match v {
                5 => {
                    return HandKind::Five;
                }
                4 => {
                    return HandKind::Four;
                }
                3 => threes += 1,
                2 => twos += 1,
                _ => {}
            }
        }
        match (threes, twos) {
            (1, 1) => HandKind::Full,
            (1, 0) => HandKind::Three,
            (0, 2) => HandKind::Pair,
            (0, 1) => HandKind::Two,
            _ => HandKind::High,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let kind = self.kind() as usize;
        let other_kind = other.kind() as usize;
        if kind < other_kind {
            Ordering::Greater
        } else if kind > other_kind {
            Ordering::Less
        } else {
            for i in 0..5 {
                if self.order[i] > other.order[i] {
                    return Ordering::Greater;
                } else if self.order[i] < other.order[i] {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

#[derive(Eq, Clone, Debug)]
struct Hand {
    cards: HashMap<u32, u32>,
    order: [u32; 5],
    bid: u32,
}

fn parse_card(card: char, joker: bool) -> u32 {
    match card {
        'T' => 10,
        'J' => {
            if joker {
                0
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        x => x.to_digit(10).unwrap(),
    }
}
