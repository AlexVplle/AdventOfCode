use aoc_runner_derive::{aoc, aoc_generator};
use std::str::SplitWhitespace;

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Vec<Vec<[u32; 3]>> {
    input
        .lines()
        .map(|line: &str| {
            line.split(": ").nth(1).map_or_else(
                || vec![],
                |game_content: &str| {
                    game_content
                        .split(';')
                        .map(|round: &str| {
                            let mut result_round: [u32; 3] = [0; 3];
                            round.split(',').for_each(|color: &str| {
                                let mut parts: SplitWhitespace = color.split_whitespace();
                                if let (Some(number_string), Some(color)) =
                                    (parts.next(), parts.next())
                                {
                                    let number: u32 =
                                        number_string.parse().expect("Expected a number");
                                    let index_color = match color {
                                        "red" => 0,
                                        "green" => 1,
                                        _ => 2,
                                    };
                                    result_round[index_color] += number;
                                }
                            });
                            result_round
                        })
                        .collect()
                },
            )
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<[u32; 3]>]) -> usize {
    input
        .into_iter()
        .enumerate()
        .filter(|(_, game)| {
            game.into_iter()
                .all(|round: &[u32; 3]| round[0] < 13 && round[1] < 14 && round[2] < 15)
        })
        .map(|(index, _)| index + 1)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<[u32; 3]>]) -> u32 {
    input.into_iter().fold(0, |acc, game| {
        let max_color: [u32; 3] = game.into_iter().fold([0; 3], |mut max, round| {
            for (index, &value) in round.iter().enumerate() {
                if max[index] < value {
                    max[index] = value;
                }
            }
            max
        });
        acc + max_color.into_iter().reduce(|acc: u32, value: u32| acc * value).unwrap()
    })
}
