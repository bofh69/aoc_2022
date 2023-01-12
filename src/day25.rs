// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

type InputType = i64;
type SolutionType = String;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '=' => -2,
                    '-' => -1,
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    _ => panic!("Unknown char {c}"),
                })
                .fold(0, |acc, n| acc * 5 + n)
        })
        .collect()
}

#[aoc(day25, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut n: InputType = data.iter().sum();
    let mut res = vec![];
    while n != 0 {
        let r = n % 5;
        if r >= 3 {
            res.push(r - 5);
            n = n / 5 + 1;
        } else {
            res.push(r);
            n /= 5;
        }
    }
    let mut s = String::new();
    for i in 0..res.len() {
        let c = match res[res.len() - i - 1] {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!("Invalid number"),
        };
        s.push(c);
    }
    s
}

#[aoc(day25, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let n: InputType = data.iter().sum();
    format!("{n}")
}
