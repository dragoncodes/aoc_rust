use rayon::prelude::*;
use std::{collections::HashMap, ops::Range, time::Instant};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let start_part_1 = Instant::now();
    let part_1_result = solution_part_1(&input);
    let part_1_time = start_part_1.elapsed();

    println!("Part 1: {} ({:?})", part_1_result, part_1_time);

    let input = std::fs::read_to_string("input.txt").unwrap();

    let start_part_2 = Instant::now();
    let part_2_result = solution_part_2(&input);
    let part_2_time = start_part_2.elapsed();

    println!("Part 2: {} ({:?})", part_2_result, part_2_time);
}

#[derive(Debug)]
struct RangeStorage {
    source: usize,
    destination: usize,
    length: usize,
}

impl RangeStorage {
    fn value_for(&self, source: usize) -> usize {
        if (self.destination..(self.destination + self.length)).contains(&source) {
            self.source + (source - self.destination)
        } else {
            source
        }
    }
}

fn solution_part_1(input: &str) -> usize {
    let mut lines = input.split("\n\n");

    let mut seeds = lines
        .nth(0)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .fold(HashMap::<usize, RangeStorage>::new(), |mut acc, x| {
            acc.insert(
                x,
                RangeStorage {
                    source: x,
                    destination: x,
                    length: 1,
                },
            );

            acc
        });

    let mut soils =
        lines
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .fold(Vec::<RangeStorage>::new(), |mut acc, line| {
                let range: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

                let soil_start = range[0] as usize;
                let seed_start = range[1] as usize;
                let count = range[2] as usize;

                acc.push(RangeStorage {
                    source: soil_start,
                    destination: seed_start,
                    length: count,
                });

                acc
            });

    let mut fertilizers =
        lines
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .fold(Vec::<RangeStorage>::new(), |mut acc, line| {
                let range: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

                let soil_start = range[0] as usize;
                let seed_start = range[1] as usize;
                let count = range[2] as usize;

                acc.push(RangeStorage {
                    source: soil_start,
                    destination: seed_start,
                    length: count,
                });

                // for i in 0..count {
                //     let soil = soil_start + i;
                //     let seed = seed_start + i;
                //
                //     if let None = soils.get_mut(&seed) {
                //         soils.insert(seed, soil);
                //     }
                // }

                acc
            });

    let mut waters =
        lines
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .fold(Vec::<RangeStorage>::new(), |mut acc, line| {
                let range: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

                let soil_start = range[0] as usize;
                let seed_start = range[1] as usize;
                let count = range[2] as usize;

                acc.push(RangeStorage {
                    source: soil_start,
                    destination: seed_start,
                    length: count,
                });

                acc
            });

    let mut lights =
        lines
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .fold(Vec::<RangeStorage>::new(), |mut acc, line| {
                let range: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

                let soil_start = range[0] as usize;
                let seed_start = range[1] as usize;
                let count = range[2] as usize;

                acc.push(RangeStorage {
                    source: soil_start,
                    destination: seed_start,
                    length: count,
                });

                acc
            });

    let mut temperatures =
        lines
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .fold(Vec::<RangeStorage>::new(), |mut acc, line| {
                let range: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

                let soil_start = range[0] as usize;
                let seed_start = range[1] as usize;
                let count = range[2] as usize;

                acc.push(RangeStorage {
                    source: soil_start,
                    destination: seed_start,
                    length: count,
                });

                acc
            });

    let mut humidities =
        lines
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .fold(Vec::<RangeStorage>::new(), |mut acc, line| {
                let range: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

                let soil_start = range[0] as usize;
                let seed_start = range[1] as usize;
                let count = range[2] as usize;

                acc.push(RangeStorage {
                    source: soil_start,
                    destination: seed_start,
                    length: count,
                });

                acc
            });

    let mut locations =
        lines
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .fold(Vec::<RangeStorage>::new(), |mut acc, line| {
                let range: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

                let soil_start = range[0] as usize;
                let seed_start = range[1] as usize;
                let count = range[2] as usize;

                acc.push(RangeStorage {
                    source: soil_start,
                    destination: seed_start,
                    length: count,
                });

                acc
            });

    seeds
        .keys()
        .map(|k| {
            // seed -> soil - > fert -> water -> light  -> temp -> hum -> locv
            let soil = soils
                .iter()
                .find_map(|s| {
                    let value = s.value_for(*k);

                    if value.eq(k) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .unwrap_or(*k);

            let fert = fertilizers
                .iter()
                .find_map(|s| {
                    let value = s.value_for(soil);

                    if value.eq(&soil) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .unwrap_or(soil);

            let water = waters
                .iter()
                .find_map(|s| {
                    let value = s.value_for(fert);

                    if value.eq(&fert) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .unwrap_or(fert);

            let light = lights
                .iter()
                .find_map(|s| {
                    let value = s.value_for(water);

                    if value.eq(&water) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .unwrap_or(water);

            let temp = temperatures
                .iter()
                .find_map(|s| {
                    let value = s.value_for(light);

                    if value.eq(&light) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .unwrap_or(light);

            let hum = humidities
                .iter()
                .find_map(|s| {
                    let value = s.value_for(temp);

                    if value.eq(&temp) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .unwrap_or(temp);

            let loc = locations
                .iter()
                .find_map(|s| {
                    let value = s.value_for(hum);

                    if value.eq(&hum) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .unwrap_or(hum);

            loc
        })
        .min()
        .unwrap()
}

fn solution_part_2(input: &str) -> usize {
    let mut lines = input.split("\n\n");

    let seeds = lines
        .nth(0)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .fold(Vec::new(), |mut acc, x| {
            for i in 0..x[1] {
                acc.push(x[0] + i);
            }

            println!("Adding {}", x[0]);
            acc
        });
    // .fold(HashMap::<usize, RangeStorage>::new(), |mut acc, x| {
    //     acc.insert(
    //         x,
    //         RangeStorage {
    //             source: x,
    //             destination: x,
    //             length: 1,
    //         },
    //     );
    //
    //     acc
    // });

    let soils =
        lines
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .fold(Vec::<RangeStorage>::new(), |mut acc, line| {
                let range: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

                let soil_start = range[0] as usize;
                let seed_start = range[1] as usize;
                let count = range[2] as usize;

                acc.push(RangeStorage {
                    source: soil_start,
                    destination: seed_start,
                    length: count,
                });

                acc
            });

    let fertilizers =
        lines
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .fold(Vec::<RangeStorage>::new(), |mut acc, line| {
                let range: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

                let soil_start = range[0] as usize;
                let seed_start = range[1] as usize;
                let count = range[2] as usize;

                acc.push(RangeStorage {
                    source: soil_start,
                    destination: seed_start,
                    length: count,
                });

                // for i in 0..count {
                //     let soil = soil_start + i;
                //     let seed = seed_start + i;
                //
                //     if let None = soils.get_mut(&seed) {
                //         soils.insert(seed, soil);
                //     }
                // }

                acc
            });

    let waters =
        lines
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .fold(Vec::<RangeStorage>::new(), |mut acc, line| {
                let range: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

                let soil_start = range[0] as usize;
                let seed_start = range[1] as usize;
                let count = range[2] as usize;

                acc.push(RangeStorage {
                    source: soil_start,
                    destination: seed_start,
                    length: count,
                });

                acc
            });

    let lights =
        lines
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .fold(Vec::<RangeStorage>::new(), |mut acc, line| {
                let range: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

                let soil_start = range[0] as usize;
                let seed_start = range[1] as usize;
                let count = range[2] as usize;

                acc.push(RangeStorage {
                    source: soil_start,
                    destination: seed_start,
                    length: count,
                });

                acc
            });

    let temperatures =
        lines
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .fold(Vec::<RangeStorage>::new(), |mut acc, line| {
                let range: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

                let soil_start = range[0] as usize;
                let seed_start = range[1] as usize;
                let count = range[2] as usize;

                acc.push(RangeStorage {
                    source: soil_start,
                    destination: seed_start,
                    length: count,
                });

                acc
            });

    let humidities =
        lines
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .fold(Vec::<RangeStorage>::new(), |mut acc, line| {
                let range: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

                let soil_start = range[0] as usize;
                let seed_start = range[1] as usize;
                let count = range[2] as usize;

                acc.push(RangeStorage {
                    source: soil_start,
                    destination: seed_start,
                    length: count,
                });

                acc
            });

    let locations =
        lines
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .fold(Vec::<RangeStorage>::new(), |mut acc, line| {
                let range: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

                let soil_start = range[0] as usize;
                let seed_start = range[1] as usize;
                let count = range[2] as usize;

                acc.push(RangeStorage {
                    source: soil_start,
                    destination: seed_start,
                    length: count,
                });

                acc
            });

    seeds
        .into_par_iter()
        .map(|k| {
            // seed -> soil - > fert -> water -> light  -> temp -> hum -> locv
            let soil = soils
                .iter()
                .find_map(|s| {
                    let value = s.value_for(k);

                    if value.eq(&k) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .unwrap_or(k);

            let fert = fertilizers
                .iter()
                .find_map(|s| {
                    let value = s.value_for(soil);

                    if value.eq(&soil) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .unwrap_or(soil);

            let water = waters
                .iter()
                .find_map(|s| {
                    let value = s.value_for(fert);

                    if value.eq(&fert) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .unwrap_or(fert);

            let light = lights
                .iter()
                .find_map(|s| {
                    let value = s.value_for(water);

                    if value.eq(&water) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .unwrap_or(water);

            let temp = temperatures
                .iter()
                .find_map(|s| {
                    let value = s.value_for(light);

                    if value.eq(&light) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .unwrap_or(light);

            let hum = humidities
                .iter()
                .find_map(|s| {
                    let value = s.value_for(temp);

                    if value.eq(&temp) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .unwrap_or(temp);

            let loc = locations
                .iter()
                .find_map(|s| {
                    let value = s.value_for(hum);

                    if value.eq(&hum) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .unwrap_or(hum);

            loc
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4        "#
            .trim();

        assert_eq!(solution_part_1(input), 35);
    }

    #[test]
    fn part2() {
        let input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4        "#
            .trim();

        assert_eq!(solution_part_2(input), 46);
    }
}
