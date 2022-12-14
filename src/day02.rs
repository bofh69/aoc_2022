// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

type InputType = (u8, u8);
type SolutionType = i32;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|line| (line.as_bytes()[0] - b'A', line.as_bytes()[2] - b'X'))
        .collect()
}

fn score(shape: u8, other: u8) -> SolutionType {
    let shapes = (3 + other - shape) % 3;
    match shapes {
        0 => 3,
        2 => 6,
        _ => 0,
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|&(other, shape)| -> SolutionType { shape as SolutionType + 1 + score(shape, other) })
        .sum()
}

fn response(other: u8, result: u8) -> u8 {
    (other + 3 + result - 1) % 3
}

#[aoc(day2, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|&(other, result)| -> SolutionType {
            let shape = response(other, result);
            shape as SolutionType + 1 + score(shape, other)
        })
        .sum()
}
