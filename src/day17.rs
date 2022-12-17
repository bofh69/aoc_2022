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

type Coordinate = i64;
type InputType = Direction;
type SolutionType = u64;

const SHAPES: [[(Coordinate, Coordinate); 5]; 5] = [
    [(0, 0), (1, 0), (2, 0), (3, 0), (3, 0)], // -
    [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // +
    [(2, 1), (2, 2), (0, 0), (1, 0), (2, 0)], // mirrored L
    [(0, 0), (0, 1), (0, 2), (0, 3), (0, 3)], // |
    [(0, 0), (0, 1), (1, 0), (1, 1), (1, 1)], // .
];

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .as_bytes()
        .iter()
        .map(|&c| {
            if c == b'<' {
                Direction::Left
            } else if c == b'>' {
                Direction::Right
            } else {
                panic!("Unknown char");
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

const TOTAL_LOOPS: u64 = 1000000000000;

#[aoc(day17, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut rocks = HashSet::new();
    let mut shape = 0;
    let mut jet_pos = 0;

    let mut top = 3;

    let mut first_loops = 0;
    let mut second_top = 0;
    let mut second_loops = 0;

    let mut n_total = 0;
    let mut wanted_pos = 0;
    let mut wanted_shape = 6;
    for which in 0..4 {
        let mut n_loops = 0;
        loop {
            n_loops += 1;
            n_total += 1;
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

            if wanted_shape == 6 && n_loops == data.len() * SHAPES.len() - 1 {
                wanted_shape = dbg!(shape);
                wanted_pos = dbg!(jet_pos);
            }
            if n_loops >= data.len() * SHAPES.len()
                && jet_pos == wanted_pos
                && shape == wanted_shape
            {
                // should be == 1, 1
                let top = top - 3;
                match which {
                    0 => {
                        first_loops = n_total;
                    }
                    1 => {
                        second_top = top;
                        second_loops = n_total;
                    }
                    2 => {
                        second_top = top - second_top;
                        second_loops = n_total - second_loops;
                    }
                    _ => panic!("Looped too long"),
                }
                break;
            } else if which == 3 {
                let remainder = (TOTAL_LOOPS - first_loops) % second_loops;
                if n_loops == remainder as usize {
                    break;
                }
            }
        }

        println!("Loops: {n_loops}");
        for y in 0..=10 {
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
        println!("   +-------+");
    }

    let divs = (TOTAL_LOOPS - first_loops) / second_loops - 2;
    dbg!(divs);

    // 1514285714288 -
    ((top as SolutionType - 3) + divs as SolutionType * dbg!(second_top) as SolutionType)
        as SolutionType
}
