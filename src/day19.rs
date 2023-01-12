// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

type CountType = u16;
type InputType = (
    u8,
    CountType,
    CountType,
    CountType,
    CountType,
    CountType,
    CountType,
);
type SolutionType = i32;

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input.lines().map(|line| sscanf::sscanf!(line, "Blueprint {u8}: Each ore robot costs {CountType} ore. Each clay robot costs {CountType} ore. Each obsidian robot costs {CountType} ore and {CountType} clay. Each geode robot costs {CountType} ore and {CountType} obsidian.").unwrap()).collect()
}

#[aoc(day19, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.len() as SolutionType
}

#[aoc(day19, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.len() as SolutionType
}
