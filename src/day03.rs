// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

type InputType = String;
type SolutionType = i32;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input.lines().map(|line| line.to_owned()).collect()
}

fn priority(c: char) -> SolutionType {
    if ('a'..='z').contains(&c) {
        1 + (c as u8 - b'a') as SolutionType
    } else {
        27 + (c as u8 - b'A') as SolutionType
    }
}

#[aoc(day3, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|line| {
            let l = line.len();
            (&line.as_str()[..l / 2], &line.as_str()[l / 2..])
        })
        .map(|(first, second)| {
            let mut result = 0;
            for c in first.chars() {
                if second.contains(c) {
                    result = priority(c);
                    break;
                }
            }
            result
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut result = 0;
    for i in (0..data.len()).step_by(3) {
        for c in data[i].chars() {
            if data[i + 1].contains(c) && data[i + 2].contains(c) {
                result += priority(c);
                break;
            }
        }
    }
    result
}
