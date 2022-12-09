// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

type InputType = (Dir, u8);
type SolutionType = i32;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|line| {
            let dir = match line.as_bytes()[0] {
                b'U' => Dir::Up,
                b'D' => Dir::Down,
                b'L' => Dir::Left,
                b'R' => Dir::Right,
                _ => unreachable!("Unknown dir: {line}"),
            };
            (dir, line[2..].parse().expect("Can't parse {line}"))
        })
        .collect()
}

fn move_head(dir: Dir, x: i32, y: i32, len: u8) -> (i32, i32) {
    match dir {
        Dir::Up => (x, y - len as i32),
        Dir::Down => (x, y + len as i32),
        Dir::Left => (x - len as i32, y),
        Dir::Right => (x + len as i32, y),
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut head_x = 0;
    let mut head_y = 0;
    let mut tail_x = 0;
    let mut tail_y = 0;
    let mut positions = std::collections::HashSet::new();
    positions.insert((0, 0));
    for (dir, len) in data {
        (head_x, head_y) = move_head(*dir, head_x, head_y, *len);
        while (head_x - tail_x).abs() > 1 || (head_y - tail_y).abs() > 1 {
            let diff_x = head_x - tail_x;
            let diff_y = head_y - tail_y;
            tail_x += diff_x.signum();
            tail_y += diff_y.signum();
            positions.insert((tail_x, tail_y));
        }
    }
    positions.len() as SolutionType
}

#[aoc(day9, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    const ROPES: usize = 10;
    let mut rope_x = [0; ROPES];
    let mut rope_y = [0; ROPES];
    let mut positions = std::collections::HashSet::new();
    positions.insert((0, 0));
    for (dir, len) in data {
        (rope_x[0], rope_y[0]) = move_head(*dir, rope_x[0], rope_y[0], *len);
        while (rope_x[0] - rope_x[1]).abs() > 1 || (rope_y[0] - rope_y[1]).abs() > 1 {
            for i in 1..ROPES {
                if (rope_x[i - 1] - rope_x[i]).abs() > 1 || (rope_y[i - 1] - rope_y[i]).abs() > 1 {
                    let diff_x = rope_x[i - 1] - rope_x[i];
                    let diff_y = rope_y[i - 1] - rope_y[i];
                    rope_x[i] += diff_x.signum();
                    rope_y[i] += diff_y.signum();
                    if i == ROPES - 1 {
                        positions.insert((rope_x[ROPES - 1], rope_y[ROPES - 1]));
                    }
                }
                if i == ROPES - 1 {
                    positions.insert((rope_x[ROPES - 1], rope_y[ROPES - 1]));
                }
            }
        }
        /*
        let min_x = *positions.iter().map(|(x, _y)| x).min().unwrap() - ROPES as i32;
        let min_y = *positions.iter().map(|(_x, y)| y).min().unwrap() - ROPES as i32;
        let max_x = *positions.iter().map(|(x, _y)| x).max().unwrap() + ROPES as i32;
        let max_y = *positions.iter().map(|(_x, y)| y).max().unwrap() + ROPES as i32;
        for y in min_y..=max_y {
            'next: for x in min_x..=max_x {
                for i in 0..ROPES {
                    if rope_x[i] == x && rope_y[i] == y {
                        print!("{}", i);
                        continue 'next;
                    }
                }
                if positions.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
        */
    }
    /*
    {
        let min_x = *positions.iter().map(|(x, _y)| x).min().unwrap() - ROPES as i32;
        let min_y = *positions.iter().map(|(_x, y)| y).min().unwrap() - ROPES as i32;
        let max_x = *positions.iter().map(|(x, _y)| x).max().unwrap() + ROPES as i32;
        let max_y = *positions.iter().map(|(_x, y)| y).max().unwrap() + ROPES as i32;
        for y in min_y..=max_y {
            'next: for x in min_x..=max_x {
                for i in 0..ROPES {
                    if rope_x[i] == x && rope_y[i] == y {
                        print!("{}", i);
                        continue 'next;
                    }
                }
                if positions.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
    */
    positions.len() as SolutionType
}
