// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
// use rayon::prelude::*;

type Coordinate = i8;
type Position = (Coordinate, Coordinate, Coordinate);
type InputType = Position;
type SolutionType = i32;

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|line| sscanf::sscanf!(line, "{Coordinate},{Coordinate},{Coordinate}").unwrap())
        .collect()
}

fn get_areas(cubes: &HashSet<Position>) -> SolutionType {
    let mut result = 0;
    for cube in cubes {
        if !cubes.contains(&(cube.0 + 1, cube.1, cube.2)) {
            result += 1;
        }
        if !cubes.contains(&(cube.0, cube.1 + 1, cube.2)) {
            result += 1;
        }
        if !cubes.contains(&(cube.0, cube.1, cube.2 + 1)) {
            result += 1;
        }
        if !cubes.contains(&(cube.0 - 1, cube.1, cube.2)) {
            result += 1;
        }
        if !cubes.contains(&(cube.0, cube.1 - 1, cube.2)) {
            result += 1;
        }
        if !cubes.contains(&(cube.0, cube.1, cube.2 - 1)) {
            result += 1;
        }
    }
    result
}

#[aoc(day18, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let cubes: HashSet<Position> = data.iter().copied().collect();
    get_areas(&cubes)
}

fn try_fill_(
    cubes: &mut HashSet<Position>,
    pos: &Position,
    min: &Position,
    max: &Position,
) -> Option<SolutionType> {
    // At a boarder
    if pos.0 - 1 < min.0 {
        return None;
    }
    if pos.1 - 1 < min.1 {
        return None;
    }
    if pos.2 - 1 < min.2 {
        return None;
    }
    if pos.0 + 1 > max.0 {
        return None;
    }
    if pos.1 + 1 > max.1 {
        return None;
    }
    if pos.2 + 1 > max.2 {
        return None;
    }

    cubes.insert(*pos);
    let mut result = 0;
    if !cubes.contains(&(pos.0 - 1, pos.1, pos.2)) {
        if let Some(n) = try_fill_(cubes, &(pos.0 - 1, pos.1, pos.2), min, max) {
            result += n - 1;
        } else {
            return None;
        }
    } else {
        result += 1;
    }
    if !cubes.contains(&(pos.0, pos.1 - 1, pos.2)) {
        if let Some(n) = try_fill_(cubes, &(pos.0, pos.1 - 1, pos.2), min, max) {
            result += n - 1;
        } else {
            return None;
        }
    } else {
        result += 1;
    }
    if !cubes.contains(&(pos.0, pos.1, pos.2 - 1)) {
        if let Some(n) = try_fill_(cubes, &(pos.0, pos.1, pos.2 - 1), min, max) {
            result += n - 1;
        } else {
            return None;
        }
    }
    if !cubes.contains(&(pos.0 + 1, pos.1, pos.2)) {
        if let Some(n) = try_fill_(cubes, &(pos.0 + 1, pos.1, pos.2), min, max) {
            result += n - 1;
        } else {
            return None;
        }
    } else {
        result += 1;
    }
    if !cubes.contains(&(pos.0, pos.1 + 1, pos.2)) {
        if let Some(n) = try_fill_(cubes, &(pos.0, pos.1 + 1, pos.2), min, max) {
            result += n - 1;
        } else {
            return None;
        }
    } else {
        result += 1;
    }
    if !cubes.contains(&(pos.0, pos.1, pos.2 + 1)) {
        if let Some(n) = try_fill_(cubes, &(pos.0, pos.1, pos.2 + 1), min, max) {
            result += n - 1;
        } else {
            return None;
        }
    } else {
        result += 1;
    }
    Some(result)
}

fn try_fill(
    cubes: &mut HashSet<Position>,
    pos: &Position,
    min: &Position,
    max: &Position,
) -> SolutionType {
    let mut spare_cubes = cubes.clone();
    if let Some(n) = try_fill_(&mut spare_cubes, pos, min, max) {
        cubes.extend(spare_cubes.iter());
        n
    } else {
        0
    }
}

#[aoc(day18, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut cubes: HashSet<Position> = data.iter().copied().collect();
    let min = {
        let min_x = cubes.iter().map(|(x, _, _)| *x).min().unwrap();
        let min_y = cubes.iter().map(|(_, y, _)| *y).min().unwrap();
        let min_z = cubes.iter().map(|(_, _, z)| *z).min().unwrap();
        (min_x, min_y, min_z)
    };
    let max = {
        let max_x = cubes.iter().map(|(x, _, _)| *x).max().unwrap();
        let max_y = cubes.iter().map(|(_, y, _)| *y).max().unwrap();
        let max_z = cubes.iter().map(|(_, _, z)| *z).max().unwrap();
        (max_x, max_y, max_z)
    };

    let mut result = get_areas(&cubes);
    for x in min.0 + 1..max.0 {
        for y in min.1 + 1..max.1 {
            for z in min.2 + 1..max.2 {
                if !cubes.contains(&(x, y, z)) {
                    result -= try_fill(&mut cubes, &(x, y, z), &min, &max);
                }
            }
        }
    }
    dbg!(result);
    // The above result became incorrect, so count again:
    get_areas(&cubes)
}
