use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day25_test.txt"), 54);
    dbg!(part1("assets/day25.txt"));
}

fn part1(file: &str) -> u64 {
    let (_interner, network) = parse_file(file);
    let random = network.nodes().next().unwrap();
    let trimmed = trim_paths(&network, random);
    let mut cluster = HashSet::new();
    cluster.insert(random);
    loop {
        let mut new_insert = false;
        for node in cluster.clone().iter() {
            for neighbor in trimmed.neighbors(*node) {
                if cluster.insert(neighbor) {
                    new_insert = true;
                }
            }
        }
        if !new_insert {
            break;
        }
    }
    ((trimmed.nodes().count() - cluster.len()) * cluster.len()) as u64
}

fn trim_paths(graph: &UnGraphMap<SymbolU32, u64>, start: SymbolU32) -> UnGraphMap<SymbolU32, u64> {
    let mut attempted = HashSet::new();
    attempted.insert(start);
    loop {
        let mut trimmed = graph.clone();
        let end = graph.nodes().find(|n| !attempted.contains(n)).unwrap();
        attempted.insert(end);
        for _ in 0..3 {
            let (path, _) = dijkstra(
                &start,
                |n| trimmed.neighbors(*n).map(|n| (n, 1)),
                |n| *n == end,
            )
            .unwrap();
            let mut prior = start;
            for node in path {
                trimmed.remove_edge(prior, node);
                prior = node;
            }
        }
        if dijkstra(
            &start,
            |n| trimmed.neighbors(*n).map(|n| (n, 1)),
            |n| *n == end,
        )
        .is_none()
        {
            return trimmed;
        }
    }
}

fn parse_file(file: &str) -> (StringInterner, UnGraphMap<SymbolU32, u64>) {
    let file = read_file(file);
    let mut interner = StringInterner::default();
    let mut network = UnGraphMap::new();
    for line in file.lines() {
        let (node, connections) = line.split_once(": ").unwrap();
        let node_sym = interner.get_or_intern(node);
        if !network.contains_node(node_sym) {
            network.add_node(node_sym);
        }
        for connection in connections.split_whitespace() {
            let conn_sym = interner.get_or_intern(connection);
            if !network.contains_node(conn_sym) {
                network.add_node(conn_sym);
            }
            network.add_edge(node_sym, conn_sym, 1);
        }
    }
    (interner, network)
}
