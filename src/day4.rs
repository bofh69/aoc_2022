// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use sscanf::sscanf;

type InputType = (u8, u8, u8, u8);
type SolutionType = usize;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|line| sscanf!(line, "{u8}-{u8},{u8}-{u8}").unwrap())
        .collect()
}
fn within(from1: u8, to1: u8, from2: u8, to2: u8) -> bool {
    from1 >= from2 && to1 <= to2
}

#[aoc(day4, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter()
        .filter(|&&(from1, to1, from2, to2)| {
            within(from1, to1, from2, to2) || within(from2, to2, from1, to1)
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.iter()
        .filter(|&&(from1, to1, from2, to2)| {
            (from1..=to1).contains(&from2)
                || (from2..=to2).contains(&from1)
                || (from1..=to1).contains(&to2)
        })
        .count()
}
