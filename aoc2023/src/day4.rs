use aoc_runner_derive::{aoc, aoc_generator};
use regex::Match;
use std::collections::HashSet;
use rayon::prelude::*;

fn match_into_hashset(match_numbers: Match) -> HashSet<u32> {
    match_numbers
        .as_str()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect::<HashSet<u32>>()
}

#[aoc_generator(day4)]
fn parse_input_day4(input: &str) -> Vec<usize> {
    let regex_card_content = regex::Regex::new(r": ([\d\s]+) \| ([\d\s]+)").unwrap();
    input
        .lines()
        .filter_map(|line: &str| regex_card_content.captures(line))
        .map(|groups| {
            (
                match_into_hashset(groups.get(1).unwrap()),
                match_into_hashset(groups.get(2).unwrap()),
            )
        })
        .map(|(winning_numbers, numbers): (HashSet<u32>, HashSet<u32>)| {
            winning_numbers.intersection(&numbers).count()
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[usize]) -> u32 {
    input
        .par_iter()
        .map(|&number_of_matching_numbers: &usize| {
            if number_of_matching_numbers > 0 {
                2_u32.pow((number_of_matching_numbers - 1) as u32)
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[usize]) -> u32 {
    let mut number_of_copies: Vec<u32> = vec![1; input.len()];
    input.into_iter().enumerate().for_each(|(index, &number_of_matching_numbers) : (usize, &usize)| {
        for i in 1..=number_of_matching_numbers {
            if index + i >= number_of_copies.len() {
                break;
            }
            number_of_copies[index + i] += number_of_copies[index];
        }
    });
    number_of_copies.into_iter().sum()
}
