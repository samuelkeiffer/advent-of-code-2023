use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day15_test.txt"), 1320);
    dbg!(part1("assets/day15.txt"));
    assert_eq!(part2("assets/day15_test.txt"), 145);
    dbg!(part2("assets/day15.txt"));
}

fn part2(file: &str) -> u64 {
    let file = read_file(file);
    let steps = file
        .split(',')
        .map(|s| {
            let (label, focal) = s.split_once(&['=', '-'][..]).unwrap();
            Step {
                kind: if s.contains('=') {
                    StepKind::Upsert
                } else {
                    StepKind::Remove
                },
                label: label.to_string(),
                bx: hash(label),
                focal: focal.parse::<u8>().ok(),
            }
        })
        .collect::<Vec<_>>();

    let bx = Box {
        order: Vec::new(),
        lenses: HashMap::new(),
    };
    let mut boxes: [Box; 256] = array::from_fn(|_| bx.clone());
    for step in steps.iter() {
        let index = step.bx as usize;
        match step.kind {
            StepKind::Remove => {
                boxes[index].order.retain(|l| l != &step.label);
                boxes[index].lenses.remove(&step.label);
            }
            StepKind::Upsert => {
                if !boxes[index].order.contains(&step.label) {
                    boxes[index].order.push(step.label.to_string());
                }
                boxes[index]
                    .lenses
                    .insert(step.label.to_string(), step.focal.unwrap());
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, bx)| {
            bx.order
                .iter()
                .enumerate()
                .map(|(j, lbl)| *bx.lenses.get(lbl).unwrap() as u64 * (j + 1) as u64)
                .sum::<u64>()
                * (i + 1) as u64
        })
        .sum()
}

#[derive(Clone)]
struct Box {
    order: Vec<String>,
    lenses: HashMap<String, u8>,
}

enum StepKind {
    Remove,
    Upsert,
}

struct Step {
    kind: StepKind,
    label: String,
    bx: u64,
    focal: Option<u8>,
}

fn part1(file: &str) -> u64 {
    let file = read_file(file);
    file.split(',').map(hash).sum()
}

fn hash(s: &str) -> u64 {
    s.chars()
        .filter_map(|c| c.as_ascii())
        .map(|c| c.to_u8() as u64)
        .fold(0, |a, b| ((a + b) * 17) % 256)
}
