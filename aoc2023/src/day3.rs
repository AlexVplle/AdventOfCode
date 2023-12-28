use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Vec<String> {
    input.lines().map(|line: &str| line.to_string()).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[String]) -> u32 {
    let mut sum: u32 = 0;
    let mut number_start_index: i32 = -1;
    let mut number_end_index: i32 = -1;
    for (index_line, line) in input.iter().enumerate() {
        let index_prev_line: usize = if index_line == 0 { 0 } else { index_line - 1 };
        let index_next_line: usize = if index_line == input.len() - 1 {
            index_line
        } else {
            index_line + 1
        };
        let mut is_adjacent: bool = false;
        for (index_char, character) in line.chars().enumerate() {
            let index_prev_char: usize = if index_char == 0 { 0 } else { index_char - 1 };
            let index_next_char: usize = if index_char == line.len() - 1 {
                index_char
            } else {
                index_char + 1
            };
            if character.is_ascii_digit() {
                if number_start_index == -1 {
                    number_start_index = index_char as i32;
                }
                number_end_index = index_char as i32;
                for current_index_line in [index_prev_line, index_line, index_next_line] {
                    for current_index_char in [index_prev_char, index_char, index_next_char] {
                        let character = input
                            .iter()
                            .nth(current_index_line)
                            .unwrap()
                            .chars()
                            .nth(current_index_char)
                            .unwrap();
                        if character != '.' && !character.is_ascii_digit() {
                            is_adjacent = true;
                        }
                    }
                }
            } else {
                if number_start_index != -1 {
                    let substring =
                        &line[(number_start_index as usize)..(number_end_index as usize + 1)];
                    if let Ok(number) = substring.parse::<u32>() {
                        if is_adjacent {
                            sum += number;
                        }
                    }
                }
                number_start_index = -1;
                is_adjacent = false;
            }
        }
        if number_start_index != -1 {
            let substring = &line[(number_start_index as usize)..(number_end_index as usize + 1)];
            if let Ok(number) = substring.parse::<u32>() {
                if is_adjacent {
                    sum += number;
                }
            }
        }
    }
    sum
}

#[aoc(day3, part2)]
fn part2(input: &[String]) -> u32 {
    let mut sum: u32 = 0;
    let mut hash_map: HashMap<(usize, usize), u32> = HashMap::new();
    let mut number_start_index: Option<usize> = None;
    let mut number_end_index: usize = 0;
    for (index_line, line) in input.iter().enumerate() {
        let mut is_adjacent: Option<(usize, usize)> = None;
        let index_prev_line: usize = if index_line == 0 { 0 } else { index_line - 1 };
        let index_next_line: usize = if index_line == input.len() - 1 {
            index_line
        } else {
            index_line + 1
        };
        for (index_char, character) in line.chars().enumerate() {
            let index_prev_char: usize = if index_char == 0 { 0 } else { index_char - 1 };
            let index_next_char: usize = if index_char == line.len() - 1 {
                index_char
            } else {
                index_char + 1
            };
            if character.is_ascii_digit() {
                number_start_index = number_start_index.or(Some(index_char));
                number_end_index = index_char;
                for current_index_line in [index_prev_line, index_line, index_next_line] {
                    let current_line: &String = &input[current_index_line];
                    for current_index_char in [index_prev_char, index_char, index_next_char] {
                        if let Some(current_char) = current_line.chars().nth(current_index_char) {
                            if current_char == '*' {
                                is_adjacent = Some((current_index_line, current_index_char));
                            }
                        }
                    }
                }
            } else {
                if let Some(start) = number_start_index {
                    let substring = &line[start..=number_end_index];
                    if let Ok(number) = substring.parse::<u32>() {
                        if let Some(coordinates) = is_adjacent {
                            if hash_map.contains_key(&coordinates) {
                                sum += number * hash_map.get(&coordinates).unwrap();
                            } else {
                                hash_map.insert(coordinates, number);
                            }
                        }
                    }
                }
                number_start_index = None;
                is_adjacent = None;
            }
        }
        if let Some(start) = number_start_index {
            let substring = &line[start..=number_end_index];
            if let Ok(number) = substring.parse::<u32>() {
                if let Some(coordinates) = is_adjacent {
                    if hash_map.contains_key(&coordinates) {
                        sum += number * hash_map.get(&coordinates).unwrap();
                    } else {
                        hash_map.insert(coordinates, number);
                    }
                }
            }
        }
    }
    sum
}
