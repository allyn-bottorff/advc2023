use std::ops::Range;

enum MapType {
    SeedToSoil,
    SoilToFert,
    FertToWater,
    WaterToLight,
    LightToTemp,
    TempToHumid,
    HumidToLoc,
}

struct Map {
    source_range: Range<i64>,
    offset: i64,
}
impl Map {
    fn from_line(line: &str) -> Self {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();

        Map {
            source_range: Range {
                start: numbers[1],
                end: numbers[1] + numbers[2],
            },
            offset: numbers[0] - numbers[1],
        }
    }
}

struct Instructions {
    seeds: Vec<i64>,
    seed_to_soil: Vec<Map>,
    soil_to_fert: Vec<Map>,
    fert_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temp: Vec<Map>,
    temp_to_humid: Vec<Map>,
    humid_to_loc: Vec<Map>,
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let instructions = parse_file(&content);

    let mut locations = Vec::new();
    for seed in &instructions.seeds {
        locations.push(seed_to_loc(seed, &instructions));
    }

    let lowest = locations.iter().min().unwrap();
    println!("lowest location: {}", lowest);
}

fn seed_to_loc(seed: &i64, instructions: &Instructions) -> i64 {
    let mut inter: i64 = *seed;
    // println!("seed: {}", seed);
    for map in &instructions.seed_to_soil {
        if map.source_range.contains(&inter) {
            inter += map.offset;
            break;
        }
    }
    // println!("soil: {}", inter);
    for map in &instructions.soil_to_fert {
        if map.source_range.contains(&inter) {
            inter += map.offset;
            break;
        }
    }
    // println!("fertilizer: {}", inter);
    for map in &instructions.fert_to_water {
        if map.source_range.contains(&inter) {
            inter += map.offset;
            break;
        }
    }
    // println!("water: {}", inter);
    for map in &instructions.water_to_light {
        if map.source_range.contains(&inter) {
            inter += map.offset;
            break;
        }
    }
    // println!("light: {}", inter);
    for map in &instructions.light_to_temp {
        if map.source_range.contains(&inter) {
            inter += map.offset;
            break;
        }
    }
    // println!("temperature: {}", inter);
    for map in &instructions.temp_to_humid {
        if map.source_range.contains(&inter) {
            inter += map.offset;
            break;
        }
    }
    // println!("humidity: {}", inter);
    for map in &instructions.humid_to_loc {
        if map.source_range.contains(&inter) {
            inter += map.offset;
            break;
        }
    }
    // println!("location: {}", inter);
    inter
}

fn parse_file(content: &String) -> Instructions {
    let mut seeds: Vec<i64> = Vec::new();
    let mut map_type: MapType = MapType::SeedToSoil;

    let mut seed_to_soil: Vec<Map> = Vec::new();
    let mut soil_to_fert: Vec<Map> = Vec::new();
    let mut fert_to_water: Vec<Map> = Vec::new();
    let mut water_to_light: Vec<Map> = Vec::new();
    let mut light_to_temp: Vec<Map> = Vec::new();
    let mut temp_to_humid: Vec<Map> = Vec::new();
    let mut humid_to_loc: Vec<Map> = Vec::new();

    for line in content.lines() {
        if line.contains("seeds:") {
            let (_, seeds_str) = line.split_once(":").unwrap();
            seeds = seeds_str
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            continue;
        }
        if line.contains("seed-to-soil map:") {
            map_type = MapType::SeedToSoil;
            continue;
        }
        if line.contains("soil-to-fertilizer map:") {
            map_type = MapType::SoilToFert;
            continue;
        }
        if line.contains("fertilizer-to-water map:") {
            map_type = MapType::FertToWater;
            continue;
        }
        if line.contains("fertilizer-to-water map:") {
            map_type = MapType::FertToWater;
            continue;
        }
        if line.contains("water-to-light map:") {
            map_type = MapType::WaterToLight;
            continue;
        }
        if line.contains("light-to-temperature map:") {
            map_type = MapType::LightToTemp;
            continue;
        }
        if line.contains("temperature-to-humidity map:") {
            map_type = MapType::TempToHumid;
            continue;
        }
        if line.contains("humidity-to-location map:") {
            map_type = MapType::HumidToLoc;
            continue;
        }
        if line == "" {
            continue;
        }

        match map_type {
            MapType::SeedToSoil => seed_to_soil.push(Map::from_line(line)),
            MapType::SoilToFert => soil_to_fert.push(Map::from_line(line)),
            MapType::FertToWater => fert_to_water.push(Map::from_line(line)),
            MapType::WaterToLight => water_to_light.push(Map::from_line(line)),
            MapType::LightToTemp => light_to_temp.push(Map::from_line(line)),
            MapType::TempToHumid => temp_to_humid.push(Map::from_line(line)),
            MapType::HumidToLoc => humid_to_loc.push(Map::from_line(line)),
        }
    }
    Instructions {
        seeds,
        seed_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humid,
        humid_to_loc,
    }
}

#[cfg(test)]
mod tests {}
