use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Vec<u32>, [Vec<(u32, u32, u32)>; 7]) {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let seeds = sections[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut transform_map: [Vec<(u32, u32, u32)>; 7] =
        [vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    for i in 1..=7 {
        transform_map[i - 1] = parse_map_section(sections[i]);
    }
    (seeds, transform_map)
}

fn parse_map_section(section: &str) -> Vec<(u32, u32, u32)> {
    section
        .lines()
        .skip(1)
        .map(|line| {
            let nums: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            (nums[0], nums[1], nums[2])
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &(Vec<u32>, [Vec<(u32, u32, u32)>; 7])) -> u32 {
    let (seeds, transform_map): &(Vec<u32>, [Vec<(u32, u32, u32)>; 7]) = input;
    if let Some(minimum_location) = seeds
        .iter()
        .map(|&seed: &u32| {
            transform_map
                .iter()
                .fold(seed, |new_seed: u32, map: &Vec<(u32, u32, u32)>| {
                    map.iter()
                        .find(|&&(_, source, range): &&(u32, u32, u32)| {
                            new_seed >= source && new_seed < source + range
                        })
                        .map_or(new_seed, |&(destination, source, _): &(u32, u32, u32)| {
                            new_seed - source + destination
                        })
                })
        })
        .min()
    {
        minimum_location
    } else {
        0
    }
}

#[aoc(day5, part2)]
fn part2(input: &(Vec<u32>, [Vec<(u32, u32, u32)>; 7])) -> u32 {
    let (seeds, transform_map): &(Vec<u32>, [Vec<(u32, u32, u32)>; 7]) = input;
    let mut new_seeds: Vec<u32> = vec![];
    let mut iter_seeds = seeds.iter();
    while let (Some(start_seed), Some(&range_seed)) = (iter_seeds.next(), iter_seeds.next()) {
        for i in 0..range_seed {
            new_seeds.push(start_seed + i);
        }
    }
    part1(&(new_seeds, transform_map.clone()))
}
