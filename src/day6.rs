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

fn solve_for(data: &[InputType], n: usize) -> SolutionType {
    let len = data.len() /* - n */; // Adding -n makes it 14% slower!
    'next: for i in 0..len {
        for j in 0..n-1 {
            for k in j + 1..n {
                if data[i + j] == data[i + k] {
                    continue 'next;
                }
            }
        }
        return i + n;
    }
    usize::MAX
}

#[aoc(day6, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    solve_for(data, 4)
}

#[aoc(day6, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    solve_for(data, 14)
}
