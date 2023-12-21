use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day20_test.txt"), 32000000);
    assert_eq!(part1("assets/day20_test2.txt"), 11687500);
    dbg!(part1("assets/day20.txt"));
    dbg!(part2("assets/day20.txt"));
}

fn part2(file: &str) -> u64 {
    let priors = ["cl", "rp", "lb", "nj"];
    priors
        .iter()
        .map(|prior| {
            let mut network = parse_file(file);
            let mut count = 0;
            loop {
                count += 1;
                let mut init = vec![(String::from("broadcaster"), false, String::from("button"))];
                if pulse_module(&mut network, &mut init, prior, false) {
                    return count;
                }
            }
        })
        .fold(1, lcm)
}

fn part1(file: &str) -> u64 {
    let mut network = parse_file(file);
    let (mut low_count, mut high_count) = (0, 0);
    for _ in 0..1000 {
        low_count += 1;
        let mut init = vec![(String::from("broadcaster"), false, String::from("button"))];
        let (low, high) = pulse(&mut network, &mut init);
        low_count += low;
        high_count += high;
    }
    low_count as u64 * high_count as u64
}

fn pulse_module(
    network: &mut HashMap<String, Module>,
    pulses: &mut Vec<(String, bool, String)>,
    test: &str,
    test_high: bool,
) -> bool {
    let set = take(pulses);
    for (dest, high, source) in set {
        // println!("{} --{}-> {}", source, high, dest);
        if let Some(module) = network.get_mut(&dest) {
            match &mut module.kind {
                ModuleKind::Broadcaster => {
                    for output in &module.outputs {
                        pulses.push((output.to_string(), high, dest.clone()));
                    }
                }
                ModuleKind::FlipFlop(state) => {
                    if !high {
                        *state = !*state;
                        for output in &module.outputs {
                            pulses.push((output.to_string(), *state, dest.clone()));
                        }
                    }
                }
                ModuleKind::Conjunction(inputs) => {
                    if let Some(in_p) = inputs.get_mut(&source) {
                        *in_p = high;
                    }
                    let send_low = inputs.values().all(|p| *p);
                    for output in &module.outputs {
                        pulses.push((output.to_string(), !send_low, dest.clone()));
                    }
                }
            }
        }
    }

    if pulses
        .iter()
        .any(|(dest, high, _)| dest == test && *high == test_high)
    {
        return true;
    }

    if !pulses.is_empty() {
        pulse_module(network, pulses, test, test_high)
    } else {
        false
    }
}

fn pulse(
    network: &mut HashMap<String, Module>,
    pulses: &mut Vec<(String, bool, String)>,
) -> (usize, usize) {
    let set = take(pulses);
    let (mut low_count, mut high_count) = (0, 0);
    for (dest, high, source) in set {
        // println!("{} --{}-> {}", source, high, dest);
        if let Some(module) = network.get_mut(&dest) {
            match &mut module.kind {
                ModuleKind::Broadcaster => {
                    for output in &module.outputs {
                        pulses.push((output.to_string(), high, dest.clone()));
                    }
                }
                ModuleKind::FlipFlop(state) => {
                    if !high {
                        *state = !*state;
                        for output in &module.outputs {
                            pulses.push((output.to_string(), *state, dest.clone()));
                        }
                    }
                }
                ModuleKind::Conjunction(inputs) => {
                    if let Some(in_p) = inputs.get_mut(&source) {
                        *in_p = high;
                    }
                    let send_low = inputs.values().all(|p| *p);
                    for output in &module.outputs {
                        pulses.push((output.to_string(), !send_low, dest.clone()));
                    }
                }
            }
        }
    }

    low_count += pulses.iter().filter(|(_, p, _)| !*p).count();
    high_count += pulses.iter().filter(|(_, p, _)| *p).count();

    if !pulses.is_empty() {
        let (low, high) = pulse(network, pulses);
        low_count += low;
        high_count += high;
    }

    (low_count, high_count)
}

fn parse_file(file: &str) -> HashMap<String, Module> {
    let file = read_file(file);
    let mut network = HashMap::new();
    let mut conjunctions = HashMap::new();
    for line in file.lines() {
        let (module, output): (String, String);
        scan!(line.bytes() => "{} -> {}", module, output);
        let outputs = output.split(',').map(|o| o.to_string()).collect::<Vec<_>>();
        let mut module_chars = module.chars();
        let (name, kind) = match module_chars.next() {
            Some('b') => (String::from("broadcaster"), ModuleKind::Broadcaster),
            Some('%') => (
                module_chars.collect::<String>(),
                ModuleKind::FlipFlop(false),
            ),
            Some('&') => {
                let name = module_chars.collect::<String>();
                conjunctions.insert(name.clone(), Vec::new());
                (name, ModuleKind::Conjunction(HashMap::new()))
            }
            _ => unreachable!(),
        };
        let module = Module { kind, outputs };
        network.insert(name, module);
    }

    for (name, module) in network.iter() {
        for output in &module.outputs {
            if let Some(inputs) = conjunctions.get_mut(output.as_str()) {
                inputs.push(name.to_string());
            }
        }
    }

    for (name, input) in conjunctions.iter() {
        if let Some(module) = network.get_mut(name) {
            if let ModuleKind::Conjunction(inputs) = &mut module.kind {
                for input in input {
                    inputs.insert(input.to_string(), false);
                }
            }
        }
    }

    network
}

#[derive(Clone, Debug)]
struct Module {
    kind: ModuleKind,
    outputs: Vec<String>,
}

#[derive(Clone, Debug)]
enum ModuleKind {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
}
