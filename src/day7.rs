// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

#[derive(Debug)]
pub enum Command {
    Cd(bool),
    Ls,
    Size(SolutionType),
    Dir,
}

type InputType = Command;
type SolutionType = i64;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|line| {
            if line.as_bytes()[0] == b'$' {
                // Command
                let mut args = line[2..].split(' ');
                if let Some(cmd) = args.next() {
                    match cmd {
                        "ls" => Command::Ls,
                        "cd" => Command::Cd(args.next().expect("cd takes too args").to_owned() != ".."),
                        _ => unreachable!("Unknown command: {}", line),
                    }
                } else {
                    unreachable!("Incorrect line: {}", line);
                }
            } else {
                let mut fields = line.split(' ');
                let first = fields.next().expect("Should be two fields");
                if "dir" == first {
                    Command::Dir
                } else {
                    Command::Size(first.parse().unwrap())
                }
            }
        })
        .collect()
}

fn find_dir_sizes(
    position: &mut usize,
    data: &[InputType],
    dir_sizes: &mut Vec<SolutionType>,
) -> SolutionType {
    let mut total = 0;
    while *position < data.len() {
        *position += 1;
        match &data[*position - 1] {
            Command::Cd(down) => {
                if !down {
                    dir_sizes.push(total);
                    return total;
                } else {
                    total += find_dir_sizes(position, data, dir_sizes);
                }
            }
            Command::Ls => (),
            Command::Dir => (),
            Command::Size(size) => {
                total += size;
            }
        }
    }
    dir_sizes.push(total);
    total
}

#[aoc(day7, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut dir_sizes = vec![];

    let _total = find_dir_sizes(&mut 0, data, &mut dir_sizes);
    dir_sizes.iter().filter(|&&n| n < 100000).sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut dir_sizes = vec![];

    let total_used = find_dir_sizes(&mut 0, data, &mut dir_sizes);
    const TOTAL_DISK: SolutionType = 70000000;
    const NEEDED_DISK: SolutionType = 30000000;
    let to_delete = total_used - (TOTAL_DISK - NEEDED_DISK);
    *dir_sizes.iter().filter(|&&n| n >= to_delete).min().unwrap()
}
