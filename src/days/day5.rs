use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day5_test.txt"), 35);
    dbg!(part1("assets/day5.txt"));
    assert_eq!(part2("assets/day5_test.txt"), 46);
    dbg!(part2("assets/day5.txt"));
}

fn part2(file: &str) -> u64 {
    let almanac = parse_file(file);
    almanac
        .locations2()
        .into_par_iter()
        .fold(|| u64::MAX, |a, b| a.min(b))
        .reduce(|| u64::MAX, |a, b| a.min(b))
}

fn part1(file: &str) -> u64 {
    let almanac = parse_file(file);
    almanac
        .locations()
        .into_iter()
        .fold(u64::MAX, |a, b| a.min(b))
}

fn parse_file(file: &str) -> Almanac {
    let mut almanac = Almanac {
        seeds: Vec::new(),
        seeds2: Vec::new(),
        seed_soil: Maps(Vec::new()),
        soil_fertilizer: Maps(Vec::new()),
        fertilizer_water: Maps(Vec::new()),
        water_light: Maps(Vec::new()),
        light_temperature: Maps(Vec::new()),
        temperature_humidity: Maps(Vec::new()),
        humidity_location: Maps(Vec::new()),
    };

    let file = read_file(file);
    let mut mode = String::new();
    for line in file.lines() {
        if line.contains(':') {
            if line.contains("seeds") {
                line.split_whitespace()
                    .filter_map(|num| num.parse::<u64>().ok())
                    .for_each(|num| almanac.seeds.push(num));
            } else {
                mode = line.to_string();
            }
        } else if mode.is_empty() || line.is_empty() {
            continue;
        } else {
            let map_params = line
                .split_whitespace()
                .filter_map(|num| num.parse::<u64>().ok())
                .collect::<Vec<_>>();
            assert_eq!(map_params.len(), 3);
            let map = Map::new(map_params[0], map_params[1], map_params[2]);
            match mode.as_str() {
                "seed-to-soil map:" => almanac.seed_soil.0.push(map),
                "soil-to-fertilizer map:" => almanac.soil_fertilizer.0.push(map),
                "fertilizer-to-water map:" => almanac.fertilizer_water.0.push(map),
                "water-to-light map:" => almanac.water_light.0.push(map),
                "light-to-temperature map:" => almanac.light_temperature.0.push(map),
                "temperature-to-humidity map:" => almanac.temperature_humidity.0.push(map),
                "humidity-to-location map:" => almanac.humidity_location.0.push(map),
                _ => {}
            }
        }
    }

    for (start, range) in almanac
        .seeds
        .iter()
        .step_by(2)
        .zip(almanac.seeds.iter().skip(1).step_by(2))
    {
        almanac.seeds2.push(*start..(start + range));
    }

    almanac
}

#[derive(Clone, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seeds2: Vec<Range<u64>>,
    seed_soil: Maps,
    soil_fertilizer: Maps,
    fertilizer_water: Maps,
    water_light: Maps,
    light_temperature: Maps,
    temperature_humidity: Maps,
    humidity_location: Maps,
}

impl Almanac {
    fn locations(&self) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|s| self.seed_soil.convert(*s))
            .map(|s| self.soil_fertilizer.convert(s))
            .map(|s| self.fertilizer_water.convert(s))
            .map(|s| self.water_light.convert(s))
            .map(|s| self.light_temperature.convert(s))
            .map(|s| self.temperature_humidity.convert(s))
            .map(|s| self.humidity_location.convert(s))
            .collect()
    }

    fn locations2(&self) -> Vec<u64> {
        self.seeds2
            .par_iter()
            .flat_map(|range| range.clone())
            .map(|s| self.seed_soil.convert(s))
            .map(|s| self.soil_fertilizer.convert(s))
            .map(|s| self.fertilizer_water.convert(s))
            .map(|s| self.water_light.convert(s))
            .map(|s| self.light_temperature.convert(s))
            .map(|s| self.temperature_humidity.convert(s))
            .map(|s| self.humidity_location.convert(s))
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Maps(Vec<Map>);

impl Maps {
    fn convert(&self, num: u64) -> u64 {
        self.0
            .iter()
            .find_map(|map| map.try_convert(num))
            .unwrap_or(num)
    }
}

#[derive(Clone, Debug)]
struct Map {
    source: Range<u64>,
    dest_start: u64,
}

impl Map {
    fn try_convert(&self, num: u64) -> Option<u64> {
        if self.source.contains(&num) {
            Some(num - self.source.start + self.dest_start)
        } else {
            None
        }
    }

    fn new(dest_start: u64, source_start: u64, range: u64) -> Self {
        Self {
            source: source_start..(source_start + range),
            dest_start,
        }
    }
}
