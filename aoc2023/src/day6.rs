use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut iter = input
        .lines()
        .map(|line: &str| line.split(':').nth(1).unwrap())
        .map(|number_line: &str| {
            number_line
                .split_whitespace()
                .map(|number_str: &str| number_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        });
    let time_array: Vec<usize> = iter.next().unwrap();
    let distance_array: Vec<usize> = iter.next().unwrap();
    time_array
        .iter()
        .zip(distance_array.iter())
        .map(|(&a, &b)| (a, b))
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &Vec<(usize, usize)>) -> usize {
    input
        .iter()
        .map(|(time, distance_limit)| {
            (1..*time)
                .map(|time_hold: usize| time_hold * (time - time_hold))
                .filter(|distance: &usize| distance > distance_limit)
                .count()
        })
        .product()
}

#[aoc(day6, part2)]
fn part2(input: &Vec<(usize, usize)>) -> usize {
    let (time_array, distance_array): (Vec<usize>, Vec<usize>) = input.iter().cloned().unzip();
    let time: usize = get_single_value_from_array(time_array);
    let distance_limit: usize = get_single_value_from_array(distance_array);
    (1..time)
        .map(|time_hold: usize| time_hold * (time - time_hold))
        .filter(|distance: &usize| distance > &distance_limit)
        .count()
}

fn get_single_value_from_array(array: Vec<usize>) -> usize {
    array
        .into_iter()
        .map(|number: usize| number.to_string())
        .reduce(|a: String, b: String| a + &b)
        .unwrap()
        .parse()
        .unwrap()
}
