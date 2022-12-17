// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
// use rayon::prelude::*;

#[derive(PartialEq)]
pub enum Direction {
    Left,
    Right,
}

type Coordinate = i16;
type InputType = Direction;
type SolutionType = i32;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .as_bytes()
        .iter()
        .map(|&c| {
            if c == b'<' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect()
}

fn add_shape(
    rocks: &mut HashSet<(Coordinate, Coordinate)>,
    top: &mut Coordinate,
    shape: &[(Coordinate, Coordinate)],
    x: Coordinate,
    y: Coordinate,
) {
    for pos in shape {
        rocks.insert((pos.0 + x, pos.1 + y));
        if pos.1 + y + 4 > *top {
            *top = pos.1 + y + 4;
        }
    }
}

const WIDTH: Coordinate = 7;

fn is_shape_fitting(
    rocks: &HashSet<(Coordinate, Coordinate)>,
    shape: &[(Coordinate, Coordinate)],
    x: Coordinate,
    y: Coordinate,
) -> bool {
    for pos in shape {
        if pos.0 + x < 0 {
            return false;
        }
        if pos.0 + x >= WIDTH {
            return false;
        }
        if pos.1 + y < 0 {
            return false;
        }
        if rocks.contains(&(pos.0 + x, pos.1 + y)) {
            return false;
        }
    }
    true
}

#[aoc(day17, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    const SHAPES: [[(Coordinate, Coordinate); 5]; 5] = [
        [(0, 0), (1, 0), (2, 0), (3, 0), (3, 0)], // -
        [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // +
        [(2, 1), (2, 2), (0, 0), (1, 0), (2, 0)], // mirrored L
        [(0, 0), (0, 1), (0, 2), (0, 3), (0, 3)], // |
        [(0, 0), (0, 1), (1, 0), (1, 1), (1, 1)], // .
    ];

    let mut rocks = HashSet::new();
    let mut shape = 0;
    let mut jet_pos = 0;

    let mut top = 3;

    for _ in 0..2022 {
        let mut x = 2;
        let mut y = top;

        loop {
            let dx = if data[jet_pos] == Direction::Left {
                -1
            } else {
                1
            };
            jet_pos = (jet_pos + 1) % data.len();
            if is_shape_fitting(&rocks, &SHAPES[shape], x + dx, y) {
                x += dx;
            }

            if is_shape_fitting(&rocks, &SHAPES[shape], x, y - 1) {
                y -= 1;
            } else {
                break;
            }
            /*
            let mut rocks2 = HashSet::new();
            let mut top2 = top;
            add_shape(&mut rocks2, &mut top2, &SHAPES[shape], x, y);
            println!();
            for y in 0..=top2 {
                let y = top2 - y;
                print!("{y:<3}|");
                for x in 0..WIDTH {
                    if rocks2.contains(&(x, y)) {
                        print!("@");
                    } else if rocks.contains(&(x, y)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("|");
            }
            println!("+-------+");
            */
        }
        add_shape(&mut rocks, &mut top, &SHAPES[shape], x, y);
        shape = (shape + 1) % SHAPES.len();
    }

    for y in 0..=top {
        let y = top - y;
        print!("{y:<3}|");
        for x in 0..WIDTH {
            if rocks.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+");

    top as SolutionType - 3
}

#[aoc(day17, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.len() as SolutionType
}
