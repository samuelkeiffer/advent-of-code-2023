use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day8_test.txt"), 2);
    dbg!(part1("assets/day8.txt"));
    assert_eq!(part2("assets/day8_test2.txt"), 6);
    dbg!(part2("assets/day8.txt"));
}

fn part2(file: &str) -> u128 {
    let network = parse_file(file);
    let init_elements: Vec<String> = network
        .nodes
        .keys()
        .filter(|e| e.chars().nth(2) == Some('A'))
        .map(|e| e.to_string())
        .collect();
    // let final_elements: Vec<String> = network.nodes.keys().filter(|e| e.chars().nth(2) == Some('Z')).map(|e| e.to_string()).collect();
    let mut map = HashMap::new();
    for element in init_elements.iter() {
        let mut end = element.clone();
        let mut count = 0;
        while end.chars().nth(2) != Some('Z') {
            end = network.step(&end, count).to_string();
            count += 1;
        }
        map.insert((element.clone(), 0), (end, count));
    }
    // loop {
    //     let mut new_inserts = Vec::new();
    //     for (new_start, offset) in map
    //         .iter()
    //         .map(|((_start, offset), (end, count))| {
    //             (end.clone(), (offset + count) % network.steps.len())
    //         })
    //         .filter(|x| !map.contains_key(x))
    //     {
    //         let mut end = new_start.clone();
    //         end = network.step(&end, 0).to_string();
    //         let mut count = 1;
    //         while end.chars().nth(2) != Some('Z') {
    //             end = network.step(&end, count).to_string();
    //             count += 1;
    //         }
    //         new_inserts.push(((new_start.clone(), offset), (end.clone(), count)));
    //     }
    //     if new_inserts.is_empty() {
    //         break;
    //     } else {
    //         for ((start, offset), (end, steps)) in new_inserts.drain(..) {
    //             map.insert((start, offset), (end, steps));
    //         }
    //     }
    // }
    dbg!(map.clone());
    // let mut steps: Vec<(String, u128)> = init_elements.iter().cloned().map(|s| (s, 0)).collect();
    // while steps.iter().any(|(_, s)| *s != steps[0].1 || *s == 0) {
    //     if let Some((element, step)) = steps.iter_mut().min_by(|a, b| a.1.cmp(&b.1)) {
    //         let (end, count) = map
    //             .get(&(
    //                 element.clone(),
    //                 (*step % network.steps.len() as u128) as usize,
    //             ))
    //             .cloned()
    //             .unwrap();
    //         *element = end.clone();
    //         *step += count as u128;
    //     }
    // }
    // steps[0].1 as u128
    map.values().map(|x| x.1 as u128).fold(1, lcm)
}

fn part1(file: &str) -> u32 {
    let network = parse_file(file);
    let mut element = "AAA".to_string();
    let mut steps = 0;
    while element != "ZZZ" {
        element = network.step(&element, steps as usize).to_string();
        steps += 1;
    }
    steps
}

fn parse_file(file: &str) -> Network {
    let file = read_file(file);
    let steps = file
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c == 'R')
        .collect();
    let nodes = file
        .lines()
        .skip(2)
        .map(|l| {
            let (a, b, c): (String, String, String);
            scan!(l.bytes() => "{} = ({}, {})", a, b, c);
            (a, (b, c))
        })
        .collect();
    Network { steps, nodes }
}

impl Network {
    fn step(&self, element: &str, steps: usize) -> &str {
        let right = self.steps[steps % self.steps.len()];
        let step = self.nodes.get(element).unwrap();
        if right {
            &step.1
        } else {
            &step.0
        }
    }
}

struct Network {
    steps: Vec<bool>,
    nodes: HashMap<String, (String, String)>,
}
