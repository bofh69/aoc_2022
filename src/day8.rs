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
    visibles.extend((0..width * height).map(|_| false));

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

    visibles.iter().filter(|&&visible| visible).count()
}

#[aoc(day8, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let width = data[0].len();
    let height = data.len();

    let mut trees = Vec::with_capacity(width * height);
    trees.extend((0..width * height).map(|_| 0));

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut count1 = 0;
            let tree_height = data[y][x];
            for ty in 0..y {
                let ty = y - ty - 1;
                count1 += 1;
                if data[ty][x] >= tree_height {
                    break;
                }
            }
            let mut count2 = 0;
            for ty in y + 1..height {
                count2 += 1;
                if data[ty][x] >= tree_height {
                    break;
                }
            }
            let mut count3 = 0;
            for tx in 0..x {
                let tx = x - tx - 1;
                count3 += 1;
                if data[y][tx] >= tree_height {
                    break;
                }
            }
            let mut count4 = 0;
            for tx in x + 1..width {
                count4 += 1;
                if data[y][tx] >= tree_height {
                    break;
                }
            }
            trees[y * width + x] = count1 * count2 * count3 * count4;
        }
    }

    *trees.iter().max().unwrap() as usize
}
