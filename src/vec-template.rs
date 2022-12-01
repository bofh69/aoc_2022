// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

type SolutionType: i32;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<SolutionType> {
    let mut result = vec![];
    result
}

#[aoc(day1, part1)]
pub fn solve_part1(data: &[SolutionType]) -> SolutionType {
    0
}

#[aoc(day1, part2)]
pub fn solve_part2(data: &[SolutionType]) -> SolutionType {
    let mut result: Vec<_> = data.into();

    result.iter().sum::<SolutionType>()
}
