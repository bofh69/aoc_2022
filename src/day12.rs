// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

type Cost = i32;
type Coordinate = i16;
type InputType = Vec<u8>;
type SolutionType = i16;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input.lines().map(|line| line.as_bytes().into()).collect()
}

#[derive(Eq, Debug)]
struct Step {
    x: Coordinate,
    y: Coordinate,
    steps: i16,
    cost: i32,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

fn find_value(data: &[InputType], value: u8) -> (Coordinate, Coordinate) {
    let width = data[0].len();

    for (y, y_data) in data.iter().enumerate() {
        for x in 0..width {
            if y_data[x] == value {
                return (
                    x.try_into().expect("A small enough number"),
                    y.try_into().expect("A small enough number"),
                );
            }
        }
    }
    panic!("The value should always be available")
}

fn get_cost(x: Coordinate, y: Coordinate, end_x: Coordinate, end_y: Coordinate) -> Cost {
    (end_x - x).abs() as Cost + (end_y - y).abs() as Cost
}

fn get_height(data: &[InputType], x: Coordinate, y: Coordinate) -> u8 {
    let height = data[y as usize][x as usize];
    if height == b'S' {
        return b'a';
    }
    if height == b'E' {
        return b'z' + 1;
    }
    height
}

#[aoc(day12, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let width = data[0].len();
    let height = data.len();

    let mut steps = std::collections::BinaryHeap::new();
    let mut visited = std::collections::HashSet::new();

    let (x, y) = find_value(data, b'S');
    let (end_x, end_y) = find_value(data, b'E');

    steps.push(Step {
        x,
        y,
        steps: 0,
        cost: 0,
    });

    while let Some(step) = steps.pop() {
        if step.x == end_x && step.y == end_y {
            return step.steps;
        }
        if visited.contains(&(step.x, step.y)) {
            continue;
        }
        // dbg!(&step);
        visited.insert((step.x, step.y));
        let current = get_height(data, step.x, step.y);

        if step.x > 0 {
            let new = get_height(data, step.x - 1, step.y);
            if new <= current + 1 {
                steps.push(Step {
                    x: step.x - 1,
                    y: step.y,
                    steps: step.steps + 1,
                    cost: step.steps as Cost + get_cost(step.x - 1, step.y, end_x, end_y),
                });
            }
        }

        if step.x < (width - 1) as Coordinate {
            let new = get_height(data, step.x + 1, step.y);
            if new <= current + 1 {
                steps.push(Step {
                    x: step.x + 1,
                    y: step.y,
                    steps: step.steps + 1,
                    cost: step.steps as Cost + get_cost(step.x + 1, step.y, end_x, end_y),
                });
            }
        }

        if step.y > 0 {
            let new = get_height(data, step.x, step.y - 1);
            if new <= current + 1 {
                steps.push(Step {
                    y: step.y - 1,
                    x: step.x,
                    steps: step.steps + 1,
                    cost: step.steps as Cost + get_cost(step.x, step.y - 1, end_x, end_y),
                });
            }
        }

        if step.y < (height - 1) as Coordinate {
            let new = get_height(data, step.x, step.y + 1);
            if new <= current + 1 {
                steps.push(Step {
                    y: step.y + 1,
                    x: step.x,
                    steps: step.steps + 1,
                    cost: step.steps as Cost + get_cost(step.x, step.y + 1, end_x, end_y),
                });
            }
        }
    }

    0
}

#[aoc(day12, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let width = data[0].len();
    let height = data.len();

    let mut steps = std::collections::BinaryHeap::new();
    let mut visited = std::collections::HashSet::new();

    let (x, y) = find_value(data, b'E');

    steps.push(Step {
        x,
        y,
        steps: 0,
        cost: 0,
    });

    while let Some(step) = steps.pop() {
        let current = get_height(data, step.x, step.y);
        if current == b'a' {
            return step.steps;
        }
        if visited.contains(&(step.x, step.y)) {
            continue;
        }
        visited.insert((step.x, step.y));

        if step.x > 0 {
            let new = get_height(data, step.x - 1, step.y);
            if new >= current - 1 {
                steps.push(Step {
                    x: step.x - 1,
                    y: step.y,
                    steps: step.steps + 1,
                    cost: step.steps as Cost,
                });
            }
        }

        if step.x < (width - 1) as Coordinate {
            let new = get_height(data, step.x + 1, step.y);
            if new >= current - 1 {
                steps.push(Step {
                    x: step.x + 1,
                    y: step.y,
                    steps: step.steps + 1,
                    cost: step.steps as Cost,
                });
            }
        }

        if step.y > 0 {
            let new = get_height(data, step.x, step.y - 1);
            if new >= current - 1 {
                steps.push(Step {
                    y: step.y - 1,
                    x: step.x,
                    steps: step.steps + 1,
                    cost: step.steps as Cost,
                });
            }
        }

        if step.y < (height - 1) as Coordinate {
            let new = get_height(data, step.x, step.y + 1);
            if new >= current - 1 {
                steps.push(Step {
                    y: step.y + 1,
                    x: step.x,
                    steps: step.steps + 1,
                    cost: step.steps as Cost,
                });
            }
        }
    }

    0
}
