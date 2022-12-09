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

type Coordinate = i16;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: Coordinate,
    y: Coordinate,
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

fn move_head(dir: Dir, pos: Position, len: u8) -> Position {
    match dir {
        Dir::Up => Position {
            x: pos.x,
            y: pos.y - len as Coordinate,
        },
        Dir::Down => Position {
            x: pos.x,
            y: pos.y + len as Coordinate,
        },
        Dir::Left => Position {
            x: pos.x - len as Coordinate,
            y: pos.y,
        },
        Dir::Right => Position {
            x: pos.x + len as Coordinate,
            y: pos.y,
        },
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    let mut positions = std::collections::HashSet::new();
    positions.insert(Position { x: 0, y: 0 });
    for (dir, len) in data {
        head = move_head(*dir, head, *len);
        while (head.x - tail.x).abs() > 1 || (head.y - tail.y).abs() > 1 {
            let diff_x = head.x - tail.x;
            let diff_y = head.y - tail.y;
            tail.x += diff_x.signum();
            tail.y += diff_y.signum();
            positions.insert(tail);
        }
    }
    positions.len() as SolutionType
}

#[aoc(day9, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    const ROPES: usize = 10;
    let mut rope = [Position { x: 0, y: 0 }; ROPES];
    let mut positions = std::collections::HashSet::new();
    positions.insert((0, 0));
    for &(dir, len) in data {
        rope[0] = move_head(dir, rope[0], len);
        while (rope[0].x - rope[1].x).abs() > 1 || (rope[0].y - rope[1].y).abs() > 1 {
            for i in 1..ROPES {
                if (rope[i - 1].x - rope[i].x).abs() > 1 || (rope[i - 1].y - rope[i].y).abs() > 1 {
                    let diff_x = rope[i - 1].x - rope[i].x;
                    let diff_y = rope[i - 1].y - rope[i].y;
                    rope[i].x += diff_x.signum();
                    rope[i].y += diff_y.signum();
                    if i == ROPES - 1 {
                        positions.insert((rope[ROPES - 1].x, rope[ROPES - 1].y));
                    }
                }
            }
        }
    }
    /*
    {
        let min_x = positions.iter().map(|pos| pos.x).min().unwrap() - ROPES as i16;
        let min_y = positions.iter().map(|pos| pos.y).min().unwrap() - ROPES as i16;
        let max_x = positions.iter().map(|pos| pos.x).max().unwrap() + ROPES as i16;
        let max_y = positions.iter().map(|pos| pos.y).max().unwrap() + ROPES as i16;
        for y in min_y..=max_y {
            'next: for x in min_x..=max_x {
                for i in 0..ROPES {
                    if rope[i].x == x && rope[i].y == y {
                        print!("{}", i);
                        continue 'next;
                    }
                }
                if positions.contains(&Position{x, y}) {
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
