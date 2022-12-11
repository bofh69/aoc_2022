// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

type Num = i32;

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Noop,
    AddX(Num),
}

type InputType = OpCode;
type SolutionType = String;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|line| {
            let mut args = line.split(' ');
            match args.next() {
                Some("noop") => OpCode::Noop,
                Some("addx") => {
                    OpCode::AddX(args.next().expect("number").parse().expect("valid number"))
                }
                _ => panic!("Unknown input: {line}"),
            }
        })
        .collect()
}

fn cycles_per_op(op: OpCode) -> i32 {
    match op {
        OpCode::Noop => 1,
        OpCode::AddX(_) => 2,
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut x = 1;
    let mut n_cycles = 0;
    let mut signal_strengths = vec![];
    let mut next_sample = 20;

    for &op in data {
        let cycles = cycles_per_op(op);
        let after = n_cycles + cycles;
        if after >= next_sample {
            let signal_strength = x * (after / 20) * 20;
            // dbg!(&(after, next_sample, signal_strength, x));
            signal_strengths.push(signal_strength);
            next_sample += 40;
        }
        match op {
            OpCode::AddX(n) => x += n,
            OpCode::Noop => (),
        }
        n_cycles = after;
    }
    signal_strengths.iter().sum::<i32>().to_string()
}

#[aoc(day10, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    let mut screen = [false; WIDTH * HEIGHT];

    let mut x = 1i32;
    let mut n_cycles = 0;

    for &op in data {
        let cycles = cycles_per_op(op);
        for _i in 0..cycles {
            let cursor_x = n_cycles % WIDTH;
            let cursor_y = (n_cycles / WIDTH) % HEIGHT;

            if ((x - 1)..=(x + 1)).contains(&(cursor_x as i32)) {
                screen[cursor_x + cursor_y * WIDTH] = true;
            }
            n_cycles += 1;
        }

        match op {
            OpCode::AddX(n) => x += n,
            OpCode::Noop => (),
        }
    }
    // screen.iter().window(WIDTH).map(|l| l.)
    let mut result = String::new();
    result.push('\n');
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            result.push(if screen[x + y * WIDTH] { '#' } else { '.' });
        }
        result.push('\n');
    }
    result
}
