use aoc_runner_derive::{aoc, aoc_generator};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet}, str::Chars,
};

#[derive(Debug)]
enum Hand {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<(String, usize)> {
    input
        .lines()
        .map(|line: &str| line.split_once(' ').unwrap())
        .map(|(cards, bid_number): (&str, &str)| (cards.to_string(), bid_number.parse().unwrap()))
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &Vec<(String, usize)>) -> usize {
    let mut mut_input = input.clone();
    mut_input.sort_by(compare_two_hand_cards);
    mut_input.into_iter().enumerate().map(|(index, value)| (index + 1) * value.1).sum()
}

fn compare_two_hand_cards(a: &(String, usize), b: &(String, usize)) -> Ordering {
    let a_type: u8 = get_hand_type(&a.0) as u8;
    let b_type: u8 = get_hand_type(&b.0) as u8;
    if a_type > b_type {
        return Ordering::Greater;
    }
    if a_type < b_type {
        return Ordering::Less;
    }
    let mut a_iter: Chars = a.0.chars();
    let mut b_iter: Chars = b.0.chars();
    while let (Some(a_char), Some(b_char)) = (a_iter.next(), b_iter.next()) {
        let rank_letter_a: u8 = a_char.get_rank_letter();
        let rank_letter_b: u8 = b_char.get_rank_letter();
        if rank_letter_a > rank_letter_b {
            return Ordering::Greater;
        }
        if rank_letter_a < rank_letter_b {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn get_hand_type(hand_cards: &String) -> Hand {
    let hash_set: HashSet<char> = HashSet::from_iter(hand_cards.chars());
    if hash_set.len() == 1 {
        return Hand::FiveOfAKind;
    }
    let hash_map: HashMap<char, usize> = hash_set
        .iter()
        .map(|&c: &char| (c, hand_cards.matches(c).count()))
        .collect();
    for (_, count_character) in hash_map {
        if count_character == 4 {
            return Hand::FourOfAKind;
        } else if count_character == 3 {
            if hash_set.len() == 2 {
                return Hand::FullHouse;
            }
            return Hand::ThreeOfAKind;
        } else if count_character == 2 {
            if hash_set.len() == 3 {
                return Hand::TwoPair;
            } else if hash_set.len() == 4 {
                return Hand::OnePair;
            }
        }
    }
    Hand::HighCard
}

trait GetRankLetter {
    fn get_rank_letter(&self) -> u8;
}

impl GetRankLetter for char {
fn get_rank_letter(&self) -> u8 {
    match self {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        _ => 12
    }
}
    
}


#[aoc(day7, part2)]
fn part2(input: &Vec<(String, usize)>) -> u32 {
    0
}
