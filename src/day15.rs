// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::RangeInclusive;

type Coordinate = i64;
type InputType = (Coordinate, Coordinate, Coordinate, Coordinate);
type SolutionType = usize;

// const LINE: Coordinate = 10;
const LINE: Coordinate = 2000000;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input.lines().map(|line|
        sscanf::sscanf!(line, "Sensor at x={Coordinate}, y={Coordinate}: closest beacon is at x={Coordinate}, y={Coordinate}").unwrap()
    ).collect()
}

fn is_overlapping(one: &RangeInclusive<Coordinate>, other: &RangeInclusive<Coordinate>) -> bool {
    one.end() + 1 == *other.start()
        || other.end() + 1 == *one.start()
        || other.contains(one.start())
        || other.contains(one.end())
        || one.contains(other.start())
        || one.contains(other.end())
}

fn join(
    one: &RangeInclusive<Coordinate>,
    other: &RangeInclusive<Coordinate>,
) -> RangeInclusive<Coordinate> {
    RangeInclusive::new(*one.start().min(other.start()), *one.end().max(other.end()))
}

fn insert_range(result: &mut Vec<RangeInclusive<Coordinate>>, range: RangeInclusive<Coordinate>) {
    let mut found = None;
    for (i, other) in result.iter().enumerate() {
        if is_overlapping(&range, other) {
            found = Some(i);
            break;
        }
    }
    if let Some(i) = found {
        let other = result.remove(i);
        let range = join(&range, &other);
        insert_range(result, range);
    } else {
        result.push(range);
    }
}

fn join_on_line(
    result: &mut Vec<RangeInclusive<Coordinate>>,
    data: &[InputType],
    line: Coordinate,
) {
    let ranges_on_line = data
        .iter()
        .map(|(x, y, x2, y2)| (x, y, (x - x2).abs() + (y - y2).abs()))
        .filter(|&(_x, y, len)| (y + len) >= line && (y - len) <= line)
        .map(|(x, y, len)| {
            let distance = (line - y).abs();
            let start = x - (len - distance);
            let end = x + (len - distance);
            start..=end
        })
        .collect::<Vec<_>>();

    for range in ranges_on_line {
        insert_range(result, range);
    }
}

#[aoc(day15, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut joined_ranges = vec![];
    join_on_line(&mut joined_ranges, data, LINE);

    let mut beacons_on_line = data
        .iter()
        .map(|(_, _, x, y)| (x, y))
        .filter(|&(_x, y)| *y == LINE)
        .filter(|(x, _y)| joined_ranges.iter().any(|range| range.contains(x)))
        .collect::<Vec<_>>();
    beacons_on_line.dedup();
    let beacons_on_line = beacons_on_line.len() as SolutionType;

    joined_ranges
        .iter()
        .map(|range| (range.end() - range.start() + 1) as SolutionType)
        .sum::<SolutionType>()
        - beacons_on_line
}

#[aoc(day15, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    const MAX_VALUE: Coordinate = 4000000;

    let mut line = vec![];
    for y in 0..=MAX_VALUE {
        line.clear();
        join_on_line(&mut line, data, y);
        let mut x = 0;
        'find_x: while x <= MAX_VALUE {
            for range in &line {
                if range.contains(&x) {
                    x = *range.end() + 1;
                    continue 'find_x;
                }
            }
            break;
        }
        if x <= MAX_VALUE {
            return (x * 4000000 + y) as SolutionType;
        }
    }
    0
}
