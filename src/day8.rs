// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

type InputType = Vec<u8>;
type SolutionType = usize;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input.lines().map(|line| line.as_bytes().into()).collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let width = data[0].len();
    let height = data.len();

    let mut visibles = Vec::with_capacity(width * height);
    // TODO: Compare with:
    // visibles.extend((0..width*height).iter().map(|_| false));
    for _i in 0..width * height {
        visibles.push(false);
    }

    for y in 0..height {
        let mut current = 0;
        for x in 0..width {
            if data[y][x] > current {
                visibles[y * width + x] = true;
                current = data[y][x]
            }
        }
        current = 0;
        for x in 0..width {
            let x = width - x - 1;
            if data[y][x] > current {
                visibles[y * width + x] = true;
                current = data[y][x]
            }
        }
    }

    for x in 0..width {
        let mut current = 0;
        for y in 0..height {
            if data[y][x] > current {
                visibles[y * width + x] = true;
                current = data[y][x]
            }
        }
        current = 0;
        for y in 0..height {
            let y = height - y - 1;
            if data[y][x] > current {
                visibles[y * width + x] = true;
                current = data[y][x]
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if visibles[y * width + x] {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    visibles.iter().filter(|&&visible| visible).count()
}

#[aoc(day8, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.len()
}
