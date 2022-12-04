// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use sscanf::sscanf;

type InputType = (u8, u8, u8, u8);
type SolutionType = usize;

fn bitset(from: u8, to: u8) -> u128 {
    let mut result = 0;
    for i in from..=to {
        result |= 1 << i;
    }
    result
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|line| sscanf!(line, "{u8}-{u8},{u8}-{u8}").unwrap())
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|&(from1, to1, from2, to2)| (bitset(from1, to1), bitset(from2, to2)))
        .filter(|&(first_set, second_set)| {
            first_set & second_set == first_set || first_set & second_set == second_set
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|&(from1, to1, from2, to2)| (bitset(from1, to1), bitset(from2, to2)))
        .filter(|&(first_set, second_set)| first_set & second_set != 0)
        .count()
}
