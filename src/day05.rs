// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use sscanf::sscanf;

#[derive(Debug)]
struct Move {
    n: u8,
    from: u8,
    to: u8,
}

#[derive(Debug)]
pub struct InputType {
    towers: [Vec<u8>; 9],
    moves: Vec<Move>,
}

type SolutionType = String;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> InputType {
    let mut iter = input.lines();
    const VAL: Vec<u8> = Vec::new();
    let mut towers = [VAL; 9];
    'line: for line in iter.by_ref() {
        if line.is_empty() {
            break;
        }
        let line = line.as_bytes();
        for i in (1..line.len()).step_by(4) {
            if line[i] == b'1' {
                continue 'line;
            }
            if line[i] != b' ' {
                towers[i / 4].insert(0, line[i]);
            }
        }
    }
    let moves = iter
        .map(|line| {
            let (n, from, to) = sscanf!(line, "move {u8} from {u8} to {u8}").unwrap();
            let from = from - 1;
            let to = to - 1;
            Move { n, from, to }
        })
        .collect();

    InputType { towers, moves }
}

#[aoc(day5, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    let mut towers = data.towers.clone();

    for mov in &data.moves {
        for _i in 0..mov.n {
            let c = towers[mov.from as usize].pop().unwrap();
            towers[mov.to as usize].push(c);
        }
    }

    let mut result = String::new();
    for tower in &towers {
        if let Some(&c) = tower.last() {
            result.push(c as char);
        }
    }

    result
}

#[aoc(day5, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    let mut towers = data.towers.clone();

    for mov in &data.moves {
        let idx = towers[mov.from as usize].len() - mov.n as usize;
        for _i in 0..mov.n {
            let c = towers[mov.from as usize][idx];
            towers[mov.from as usize].remove(idx);
            towers[mov.to as usize].push(c);
        }
    }

    let mut result = String::new();
    for tower in &towers {
        if let Some(&c) = tower.last() {
            result.push(c as char);
        }
    }

    result
}
