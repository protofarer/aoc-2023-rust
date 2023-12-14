#![allow(unused)]
use crate::{get_string_from_input, Solver};
use rayon::prelude::*;
use std::{io::BufRead, sync::Arc};

// SOLN: 51752125 (~1ns w/o rayon, ~2-3ns w/ rayon)
// 4375167438 high :: converter.convert was wrapping because the delta
// calculation was wrong, should be `dst - src`
fn first(input: &mut dyn BufRead) -> String {
    let (
        mut seed_to_soil,
        mut soil_to_fert,
        mut fert_to_water,
        mut water_to_light,
        mut light_to_temp,
        mut temp_to_humid,
        mut humid_to_loc,
    ) = gen_maps();

    let mut seeds = process_input(
        input,
        parse_seed_line,
        &mut seed_to_soil,
        &mut soil_to_fert,
        &mut fert_to_water,
        &mut water_to_light,
        &mut light_to_temp,
        &mut temp_to_humid,
        &mut humid_to_loc,
    );

    // w/o rayon
    let mut locations = vec![];
    for seed in seeds {
        let location =
            humid_to_loc.convert(temp_to_humid.convert(light_to_temp.convert(
                water_to_light.convert(
                    fert_to_water.convert(soil_to_fert.convert(seed_to_soil.convert(seed))),
                ),
            )));
        locations.push(location);
    }

    // w/ rayon
    // let locations: Vec<usize> = seeds
    //     .par_iter()
    //     .map(|&seed| {
    //         humid_to_loc.convert(temp_to_humid.convert(light_to_temp.convert(
    //             water_to_light.convert(
    //                 fert_to_water.convert(soil_to_fert.convert(seed_to_soil.convert(seed))),
    //             ),
    //         )))
    //     })
    //     .collect();

    let min_location = locations.iter().min().unwrap();

    min_location.to_string()
}

// SOLN: 12,634,632 3min w/ rayon (elapsed 156,000ns didnt count other threads)
// 3_244_927 low (w/o rayon ~20min)

// seeds vec generation analysis
// seed vec population speed (after calc seed count):
// - w/o rayon 2,100,000,000 13,000ns (possible error undercount seeds)
// - w/ rayon 2,104,769,314 54,000ns (correctly collects seeds at cost of being slower)

// sed vec pop speed (w/o seed count for vec with_cap):
// - w/o rayon 2,104,769,314 43,000ns
// - w/ rayon _ 55,000ns

// seeds counted before populating, no rayon 42,000ns
// Conclusion: Vec::with_capacity by counting seeds before filling vec offers little to no advantage

fn second(input: &mut dyn BufRead) -> String {
    let (
        mut seed_to_soil,
        mut soil_to_fert,
        mut fert_to_water,
        mut water_to_light,
        mut light_to_temp,
        mut temp_to_humid,
        mut humid_to_loc,
    ) = gen_maps();

    let mut seeds = process_input(
        input,
        parse_seed_ranges_line,
        &mut seed_to_soil,
        &mut soil_to_fert,
        &mut fert_to_water,
        &mut water_to_light,
        &mut light_to_temp,
        &mut temp_to_humid,
        &mut humid_to_loc,
    );

    // * w/ rayon
    let locations: Vec<usize> = seeds
        .par_iter()
        .map(|&seed| {
            convert(
                seed,
                &seed_to_soil,
                &soil_to_fert,
                &fert_to_water,
                &water_to_light,
                &light_to_temp,
                &temp_to_humid,
                &humid_to_loc,
            )
        })
        .collect();

    let min_location = locations.iter().min().unwrap();
    min_location.to_string()

    // "".to_string()
}

#[derive(Clone)]
struct Converter {
    pub src_start: usize,
    src_end: usize,
    conversion_delta: i64,
}

impl Converter {
    fn new(dst_start: usize, src_start: usize, range: usize) -> Self {
        Converter {
            src_start,
            src_end: src_start + range - 1,
            conversion_delta: dst_start as i64 - src_start as i64,
        }
    }
    fn can_handle_src(&self, src: usize) -> bool {
        if src >= self.src_start && src <= self.src_end {
            true
        } else {
            false
        }
    }
    fn convert(&self, x: usize) -> usize {
        let interm = (x as i64 + self.conversion_delta);
        if interm < 0 {
            println!("WRAP! x: {}  delta: {}", x, self.conversion_delta);
        }
        let out = interm as usize;
        out
    }
}

#[derive(Clone)]
struct Map {
    converters: Vec<Converter>,
}

impl Map {
    fn new() -> Self {
        Map { converters: vec![] }
    }
    fn convert(&self, src: usize) -> usize {
        // try converting via maps, if no map exists, do 1:1 (aka simply return src)
        if let Some(converter) = self.converters.iter().find(|c| c.can_handle_src(src)) {
            converter.convert(src)
        } else {
            src
        }
    }
    fn add_converter(&mut self, dst_start: usize, src_start: usize, range: usize) {
        let converter = Converter::new(dst_start, src_start, range);
        self.converters.push(converter);
    }
    fn converters(&self) -> &Vec<Converter> {
        &self.converters
    }
}

fn gen_maps() -> (Map, Map, Map, Map, Map, Map, Map) {
    (
        Map::new(),
        Map::new(),
        Map::new(),
        Map::new(),
        Map::new(),
        Map::new(),
        Map::new(),
    )
}

fn parse_seed_line(line: &str) -> Vec<usize> {
    line.split_once(':')
        .unwrap_or(("", ""))
        .1
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}

fn parse_seed_ranges_line(line: &str) -> Vec<usize> {
    let numbers: Vec<usize> = line
        .split_once(':')
        .unwrap_or(("", ""))
        .1
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    // * no seed counting, no rayon
    let mut seeds: Vec<usize> = vec![];

    for chunk in numbers.chunks(2) {
        if let [start, length] = chunk {
            seeds.extend(*start..*start + *length);
        }
    }

    seeds
}

fn gen_collection_of_seeds(start: usize, length: usize) -> Vec<usize> {
    let end = start + length;
    (start..end).collect()
}

fn process_input<F>(
    input: &mut dyn BufRead,
    parse_seeds_fn: F,
    seed_to_soil: &mut Map,
    soil_to_fert: &mut Map,
    fert_to_water: &mut Map,
    water_to_light: &mut Map,
    light_to_temp: &mut Map,
    temp_to_humid: &mut Map,
    humid_to_loc: &mut Map,
) -> Vec<usize>
where
    F: Fn(&str) -> Vec<usize>,
{
    let mut current_map: Option<&mut Map> = None;
    let mut seeds = vec![];

    let input_string = get_string_from_input(input);
    let mut lines = input_string.lines();

    let seed_line = lines.next().unwrap();
    seeds = parse_seeds_fn(seed_line);

    for line in lines {
        let map_header = if line.contains(':') {
            let content = line.split_once(':').unwrap().0;
            Some(content)
        } else {
            None
        };

        match map_header.unwrap_or("") {
            "seed-to-soil map" => {
                current_map = Some(seed_to_soil);
            }
            "soil-to-fertilizer map" => {
                current_map = Some(soil_to_fert);
            }
            "fertilizer-to-water map" => {
                current_map = Some(fert_to_water);
            }
            "water-to-light map" => {
                current_map = Some(water_to_light);
            }
            "light-to-temperature map" => {
                current_map = Some(light_to_temp);
            }
            "temperature-to-humidity map" => {
                current_map = Some(temp_to_humid);
            }
            "humidity-to-location map" => {
                current_map = Some(humid_to_loc);
            }
            // no header, is either line of numbers or empty line
            "" => {
                if line.len() > 0 {
                    let numbers: Vec<usize> = line
                        .trim()
                        .split_whitespace()
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect();
                    if let Some(ref mut map) = current_map {
                        map.add_converter(numbers[0], numbers[1], numbers[2]);
                    }
                }
            }
            _ => {
                // should never execute
                println!("else match, line: {}", line);
            }
        }
    }
    seeds
}

fn convert(
    seed: usize,
    seed_to_soil: &Map,
    soil_to_fert: &Map,
    fert_to_water: &Map,
    water_to_light: &Map,
    light_to_temp: &Map,
    temp_to_humid: &Map,
    humid_to_loc: &Map,
) -> usize {
    humid_to_loc.convert(
        temp_to_humid.convert(
            light_to_temp.convert(
                water_to_light.convert(
                    fert_to_water.convert(soil_to_fert.convert(seed_to_soil.convert(seed))),
                ),
            ),
        ),
    )
}

fn calculate_locations_from_seeds(
    seeds: &[usize],
    seed_to_soil: Map,
    soil_to_fert: Map,
    fert_to_water: Map,
    water_to_light: Map,
    light_to_temp: Map,
    temp_to_humid: Map,
    humid_to_loc: Map,
) -> Vec<usize> {
    seeds
        .iter()
        .map(|s| {
            convert(
                *s,
                &seed_to_soil,
                &soil_to_fert,
                &fert_to_water,
                &water_to_light,
                &light_to_temp,
                &temp_to_humid,
                &humid_to_loc,
            )
        })
        .collect()
}

pub const SOLVERS: &[Solver] = &[first, second];
