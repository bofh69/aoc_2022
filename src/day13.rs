// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum List {
    List(Vec<List>),
    Number(u8),
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Lowest number wins.
            (List::Number(a), List::Number(b)) => a.cmp(b),

            // Shortest list wins with lower/equal contents.
            (List::List(a), List::List(b)) => {
                for i in 0..a.len() {
                    if i >= b.len() {
                        return Ordering::Greater;
                    }
                    let result = a[i].cmp(&b[i]);
                    if result != Ordering::Equal {
                        return result;
                    }
                }
                a.len().cmp(&b.len())
            }

            // Numbers are promoted to lists and compared:
            (List::Number(a), List::List(_)) => List::List(vec![List::Number(*a)]).cmp(other),
            (List::List(_), List::Number(b)) => self.cmp(&List::List(vec![List::Number(*b)])),
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type InputType = (List, List);
type SolutionType = i32;

fn is_digit(c: u8) -> bool {
    (b'0'..=b'9').contains(&c)
}

fn parse_list_members(line: &[u8], pos: &mut usize) -> List {
    let mut result = vec![];
    while line[*pos] != b']' {
        if line[*pos] == b'[' {
            result.push(parse_bytes(line, pos));
            if line[*pos] == b',' {
                *pos += 1;
                continue;
            }
        } else if is_digit(line[*pos]) {
            result.push(parse_number(line, pos));
            if line[*pos] == b',' {
                *pos += 1;
                continue;
            }
        }
    }
    List::List(result)
}

fn parse_number(line: &[u8], pos: &mut usize) -> List {
    let mut result = 0;
    while is_digit(line[*pos]) {
        result = result * 10 + (line[*pos] - b'0');
        *pos += 1;
    }
    List::Number(result)
}

fn parse_bytes(line: &[u8], pos: &mut usize) -> List {
    if line[*pos] == b'[' {
        *pos += 1;
        let result = parse_list_members(line, pos);
        if line[*pos] != b']' {
            panic!("Expected ] as position {pos}");
        }
        *pos += 1;
        result
    } else if is_digit(line[*pos]) {
        parse_number(line, pos)
    } else {
        panic!("Unknown data {}", line[*pos]);
    }
}

fn parse_line(line: &str) -> List {
    let line = line.as_bytes();
    parse_bytes(line, &mut 0)
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    let mut result = vec![];
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let line2 = lines.next().unwrap();
        let line = parse_line(line);
        let line2 = parse_line(line2);
        result.push((line, line2));
        lines.next();
    }
    result
}

fn right_order(a: &List, b: &List) -> Option<bool> {
    match a.cmp(b) {
        Ordering::Less => Some(true),
        Ordering::Equal => None,
        Ordering::Greater => Some(false),
    }
}

#[aoc(day13, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut result = 0;
    for (i, pair) in data.iter().enumerate() {
        if right_order(&pair.0, &pair.1).unwrap() {
            let i = i + 1;
            result += i;
        }
    }
    result as SolutionType
}

#[aoc(day13, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut rows = vec![];
    for pair in data {
        rows.push(pair.0.clone());
        rows.push(pair.1.clone());
    }
    let first_delim = parse_line("[[2]]");
    let second_delim = parse_line("[[6]]");
    let mut first = 1;
    let mut second = 2;
    for row in rows {
        if row < first_delim {
            first += 1;
            second += 1;
        } else if row < second_delim {
            second += 1;
        }
    }
    (first * second) as SolutionType
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn empty() {
        assert_eq!(List::List(vec![]), parse_line("[]"));
    }

    #[test]
    pub fn list_with_number() {
        assert_eq!(List::List(vec![List::Number(1)]), parse_line("[1]"));
    }

    #[test]
    pub fn list_with_numbers() {
        assert_eq!(
            List::List(vec![List::Number(1), List::Number(2)]),
            parse_line("[1,2]")
        );
    }

    #[test]
    pub fn list_with_empty_list() {
        assert_eq!(List::List(vec![List::List(vec![])]), parse_line("[[]]"));
    }

    #[test]
    pub fn list_with_list_with_number() {
        assert_eq!(
            List::List(vec![List::List(vec![List::Number(10)])]),
            parse_line("[[10]]")
        );
    }

    #[test]
    pub fn list_with_number_and_empty_list() {
        assert_eq!(
            List::List(vec![List::Number(7), List::List(vec![])]),
            parse_line("[7,[]]")
        );
    }

    #[test]
    pub fn list_with_empty_list_and_number() {
        assert_eq!(
            List::List(vec![List::List(vec![]), List::Number(7)]),
            parse_line("[[],7]")
        );
    }
}
