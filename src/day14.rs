// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

const START_X: Coordinate = 500;
const START_Y: Coordinate = 0;

type Coordinate = i16;
type InputType = Vec<(Coordinate, Coordinate)>;
type SolutionType = i32;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| sscanf::sscanf!(pair, "{Coordinate},{Coordinate}").expect("x,y"))
                .collect()
        })
        .collect()
}

#[derive(Debug)]
enum Movement {
    Down,
    Left,
    Right,
    Blocked,
}

fn can_move(
    world: &std::collections::HashSet<(Coordinate, Coordinate)>,
    x: Coordinate,
    y: Coordinate,
) -> Movement {
    if !world.contains(&(x, y + 1)) {
        Movement::Down
    } else if !world.contains(&(x - 1, y + 1)) {
        Movement::Left
    } else if !world.contains(&(x + 1, y + 1)) {
        Movement::Right
    } else {
        Movement::Blocked
    }
}

fn output_world(
    world: &std::collections::HashSet<(Coordinate, Coordinate)>,
    stones: &std::collections::HashSet<(Coordinate, Coordinate)>,
) {
    let min_x = stones.iter().map(|pos| pos.0).min().unwrap();
    let max_x = stones.iter().map(|pos| pos.0).max().unwrap();
    let max_y = stones.iter().map(|pos| pos.1).max().unwrap();

    for y in 0..=max_y {
        print!("{y:<2}");
        for x in min_x..=max_x {
            if y == START_Y && x == START_X {
                print!("+");
            } else if stones.contains(&(x, y)) {
                print!("#");
            } else if world.contains(&(x, y)) {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[aoc(day14, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut world = std::collections::HashSet::new();
    for segment in data {
        for i in 1..segment.len() {
            let last_segment = segment[i - 1];
            let len_x = segment[i].0 - last_segment.0;
            let len_y = segment[i].1 - last_segment.1;
            for j in 0..=(len_x.abs() + len_y.abs()) {
                world.insert((
                    last_segment.0 + j * len_x.signum(),
                    last_segment.1 + j * len_y.signum(),
                ));
            }
        }
    }

    let max_y = *data
        .iter()
        .map(|segment| segment.iter().map(|(_x, y)| y).max().unwrap())
        .max()
        .unwrap();

    let stones = world.clone();

    let mut y = START_Y;
    while y < max_y {
        let mut x = START_X;

        'fall: while y < max_y {
            match can_move(&world, x, y) {
                Movement::Down => {
                    y += 1;
                }
                Movement::Left => {
                    x -= 1;
                    y += 1;
                }
                Movement::Right => {
                    x += 1;
                    y += 1;
                }
                Movement::Blocked => {
                    world.insert((x, y));
                    y = START_Y;
                    break 'fall;
                }
            }
        }
    }

    output_world(&world, &stones);
    (world.len() - stones.len()) as SolutionType
}

#[aoc(day14, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.len() as SolutionType
}
