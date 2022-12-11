// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use sscanf::sscanf;

type ItemType = i64;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(Option<ItemType>),
    Mul(Option<ItemType>),
}

#[derive(Debug, Clone)]
pub struct Monkey {
    id: u8,
    items: Vec<ItemType>,
    op: Operation,
    divisible: ItemType,
    when_true: u8,
    when_false: u8,
    inspections: SolutionType,
}

impl Default for Monkey {
    fn default() -> Self {
        Self {
            id: 255,
            items: vec![],
            op: Operation::Add(None),
            divisible: 0,
            when_true: 255,
            when_false: 255,
            inspections: 0,
        }
    }
}

type InputType = Monkey;
type SolutionType = ItemType;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    let mut lines = input.lines();
    let mut result = vec![];
    let mut monkey = Monkey::default();
    while let Some(line) = lines.next() {
        monkey.id = sscanf!(line, "Monkey {u8}:").expect("monkey header");

        let line = lines.next().expect("Items");
        monkey.items.extend(line.split(' ').skip(4).map(|n| {
            let n_len = n.len();
            if n.as_bytes()[n_len - 1] == b',' {
                n[0..n_len - 1].parse::<ItemType>().expect("item number")
            } else {
                n.parse::<ItemType>().expect("item number")
            }
        }));

        let line = lines.next().expect("Operation");
        let params = sscanf!(line, "  Operation: new = old {char} {str}").expect("operation terms");
        let param = if params.1 == "old" {
            None
        } else {
            Some(params.1.parse::<ItemType>().expect("valid number"))
        };
        monkey.op = if params.0 == '+' {
            Operation::Add(param)
        } else {
            Operation::Mul(param)
        };

        let line = lines.next().expect("Operation");
        monkey.divisible =
            sscanf!(line, "  Test: divisible by {ItemType}").expect("divisible number");

        let line = lines.next().expect("If true");
        monkey.when_true =
            sscanf!(line, "    If true: throw to monkey {u8}").expect("monkey number");

        let line = lines.next().expect("If false");
        monkey.when_false =
            sscanf!(line, "    If false: throw to monkey {u8}").expect("monkey number");

        result.push(monkey);
        monkey = Monkey::default();
        lines.next();
    }
    result
}

#[aoc(day11, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut data: Vec<Monkey> = data.into();

    for _round in 0..20 {
        for monkey_nr in 0..data.len() {
            while let Some(item) = data[monkey_nr].items.pop() {
                let mut item = item;
                match data[monkey_nr].op {
                    Operation::Mul(None) => item *= item,
                    Operation::Add(None) => item += item,
                    Operation::Mul(Some(n)) => item *= n,
                    Operation::Add(Some(n)) => item += n,
                }
                item /= 3;
                let result = if item % data[monkey_nr].divisible == 0 {
                    data[monkey_nr].when_true as usize
                } else {
                    data[monkey_nr].when_false as usize
                };
                data[result].items.push(item);
                data[monkey_nr].inspections += 1;
            }
        }
    }
    let mut inspections: Vec<_> = data.iter().map(|monkey| monkey.inspections).collect();
    inspections.sort();
    inspections.iter().rev().take(2).product()
}

#[aoc(day11, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut data: Vec<Monkey> = data.into();

    let modulos = data
        .iter()
        .map(|monkey| monkey.divisible)
        .product::<ItemType>();
    // dbg!(modulos);

    for _round in 0..10000 {
        /*
        if round > 0 && (round % 1000 == 0 || round == 1 || round == 20) {
            println!("== After round {round} ==");
            let inspections : Vec<_> = data.iter().map(|monkey| monkey.inspections).collect();
            for i in 0..inspections.len() {
                println!("Monkey {i} inspected items {} times.", inspections[i]);
            }
            println!();
        }
        */
        for monkey_nr in 0..data.len() {
            while let Some(item) = data[monkey_nr].items.pop() {
                let mut item = item;
                match data[monkey_nr].op {
                    Operation::Mul(None) => item *= item,
                    Operation::Add(None) => item += item,
                    Operation::Mul(Some(n)) => item *= n,
                    Operation::Add(Some(n)) => item += n,
                }
                item %= modulos;
                let result = if item % data[monkey_nr].divisible == 0 {
                    data[monkey_nr].when_true as usize
                } else {
                    data[monkey_nr].when_false as usize
                };
                data[result].items.push(item);
                data[monkey_nr].inspections += 1;
            }
        }
    }
    let a = data
        .iter()
        .map(|m| m.inspections)
        .max()
        .expect("One number");
    let b = data
        .iter()
        .map(|m| m.inspections)
        .filter(|&n| n != a)
        .max()
        .expect("One number");
    a * b
}
