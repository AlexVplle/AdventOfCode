use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[String]) -> u32 {
    let mut sum: u32 = 0;
    input.into_iter().for_each(|string: &String| {
        let chars: Vec<char> = string.chars().filter(|c: &char| c.is_digit(10)).collect();
        if let (Some(first_number_string), Some(last_number_string)) = (chars.first(), chars.last())
        {
            if let (Some(first_number), Some(last_number)) = (
                first_number_string.to_digit(10),
                last_number_string.to_digit(10),
            ) {
                sum += first_number * 10 + last_number;
            }
        }
    });
    sum
}

#[aoc(day1, part2)]
fn part2(input: &[String]) -> u32 {
    let translate_vector: Vec<&str> = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum: u32 = 0;
    input.into_iter().for_each(|string: &String| {
        let mut number_vector: Vec<u32> = vec![];

        let mut string_cloned: String = string.clone();

        while !string_cloned.is_empty() {
            if let Some(first_char) = string_cloned.chars().nth(0) {
                if first_char.is_digit(10) {
                    number_vector.push(first_char.to_digit(10).unwrap())
                } else {
                    for (key, value) in translate_vector.iter().enumerate() {
                        if string_cloned.starts_with(value) {
                            number_vector.push(key as u32);
                            break;
                        }
                    }
                }
                string_cloned = string_cloned[1..].to_string();
            }
        }
        if let (Some(first_number), Some(last_number)) =
            (number_vector.first(), number_vector.last())
        {
            sum += first_number * 10 + last_number;
        }
    });
    sum
}
