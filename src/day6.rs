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

fn solve_for<const N: usize>(data: &[InputType]) -> SolutionType {
    for (pos, window) in data.windows(N).enumerate() {
        if !window
            .iter()
            .enumerate()
            .any(|(win_pos, c)| window[win_pos + 1..].contains(c))
        {
            return pos + N;
        }
    }
    usize::MAX
}

#[aoc(day6, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    solve_for::<4>(data)
}

#[aoc(day6, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    solve_for::<14>(data)
}
