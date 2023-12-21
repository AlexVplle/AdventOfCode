use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Vec<Vec<[u32; 3]>> {
    let mut result: Vec<Vec<[u32; 3]>> = vec![];
    input.lines().for_each(|line: &str| {
        let first_split: Vec<&str> = line.split(": ").collect();
        let game_content: &str = first_split[1].trim();
        let mut game: Vec<[u32; 3]> = vec![];
        let round_content: Vec<&str> = game_content.split(';').collect();
        for round in round_content {
            let mut result_round: [u32; 3] = [0, 0, 0];
            let round_array: Vec<&str> =
                round.split(',').map(|string: &str| string.trim()).collect();
            for color in round_array {
                let parts: Vec<&str> = color.split_whitespace().collect();
                let (number_string, color) = match parts.as_slice() {
                    [num, col] => (*num, *col),
                    _ => panic!("Bug"),
                };
                let number: u32 = number_string.parse().unwrap();
                let index_color: usize = match color {
                    "red" => 0,
                    "green" => 1,
                    _ => 2,
                };
                result_round[index_color] += number;
            }
            game.push(result_round);
        }
        result.push(game);
    });
    result
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<[u32; 3]>]) -> u32 {
    let mut result : u32 = 0;
    for (index, game) in input.iter().enumerate() {
        let mut good_game: bool = true;
        for round in game {
            if round[0] > 12 || round[1] > 13 || round[2] > 14 {
                good_game = false;
                break;
            }
        }
        if good_game {
            result += (index + 1) as u32;
        }
    }
    result
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<[u32; 3]>]) -> u32 {
    let mut result: u32 = 0;
    for game in input {
        let mut max_color: [u32; 3] = [0, 0, 0];
        for round in game {
            for (index, value) in round.iter().enumerate() {
                if max_color[index] < *value {
                    max_color[index] = *value;
                }
            }
        }
        result += max_color[0] * max_color[1] * max_color[2];
    }
    result
}
