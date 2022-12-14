// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

type SolutionType = i32;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<SolutionType> {
    let mut result = vec![];
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            result.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<SolutionType>().unwrap();
        }
    }
    result
}

#[aoc(day1, part1)]
pub fn solve_part1(data: &[SolutionType]) -> SolutionType {
    *data.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(data: &[SolutionType]) -> SolutionType {
    let mut result: Vec<_> = data.into();

    result.sort();
    result.iter().rev().take(3).sum::<SolutionType>()
}
