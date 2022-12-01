use aoc_runner_derive::{aoc, aoc_generator};
// use rayon::prelude::*;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    let mut result = vec!();
    let mut sum = 0;
    for line in input.lines() {
        if line == "" {
            result.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    result
}

#[aoc(day1, part1)]
pub fn solve_part1(data: &[i32]) -> i32 {
    *data.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(data: &[i32]) -> i32 {
    let mut result : Vec<_> = data.into();

    result.sort();
    result.iter().rev().take(3).sum::<i32>()
}
