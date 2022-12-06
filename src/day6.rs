// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

type InputType = u8;
type SolutionType = usize;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input.as_bytes().into()
}

#[aoc(day6, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    'next: for i in 0..data.len() - 4 {
        for j in 0..3 {
            for k in j + 1..4 {
                if data[i + j] == data[i + k] {
                    continue 'next;
                }
            }
        }
        return i + 4;
    }
    usize::MAX
}

#[aoc(day6, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    'next: for i in 0..data.len() - 4 {
        for j in 0..13 {
            for k in j + 1..14 {
                if data[i + j] == data[i + k] {
                    continue 'next;
                }
            }
        }
        return i + 14;
    }
    usize::MAX
}
