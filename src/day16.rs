// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::collections::BinaryHeap;

type Flow = i32;
type InputType = (String, Flow, Vec<String>);
type SolutionType = i32;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|line| sscanf::sscanf!(line, "Valve {str} has flow rate={Flow}; {str}").unwrap())
        .map(|(name, flow, valves)| {
            // Valve AA has flow rate=0; tunnel(s) lead to valve(s) DD, II, BB

            let mut valves = valves.split(' ');
            for _ in 0..4 {
                valves.next();
            }
            let valves = valves
                .map(|s| String::from_utf8(s.as_bytes()[0..2].to_vec()).unwrap())
                .collect();

            (name.to_owned(), flow, valves)
        })
        .collect()
}

fn find_movement_costs(movement_costs: &mut [u8], edges: &HashMap<u8, Vec<u8>>) {
    let nodes = edges.len();
    for start in 0..nodes {
        movement_costs[start * nodes + start] = 0;
        let mut current = BinaryHeap::<(u8, u8)>::new();
        current.push((0, start as u8));
        while let Some((cost, from)) = current.pop() {
            for &to in edges.get(&from).unwrap() {
                if cost + 1 < movement_costs[start * nodes + to as usize] {
                    movement_costs[start * nodes + to as usize] = cost + 1;
                    movement_costs[start * nodes + to as usize] = cost + 1;
                    current.push((cost + 1, to));
                }
            }
        }
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    // Translate strings to numbers:
    let mut name_mapping = HashMap::new();
    for (i, name) in data.iter().map(|(name, _, _)| name).enumerate() {
        name_mapping.insert(name, i as u8);
    }
    let name_mapping = name_mapping;

    // Translate edges to be from u8 to Vec<u8>:
    let mut edges = HashMap::new();
    let pressure_for_valve: HashMap<u8, _> = data
        .iter()
        .map(|(name, cost, valves)| {
            let from = *name_mapping.get(name).unwrap();
            edges.insert(
                from,
                valves
                    .iter()
                    .map(|valve| *name_mapping.get(valve).unwrap())
                    .collect::<Vec<_>>(),
            );
            (from, cost)
        })
        .collect();
    let edges = edges;

    let n_edges = edges.len();

    // Find the smallest cost to move from one valve to another:
    let mut movement_costs = Vec::with_capacity(n_edges * n_edges);
    for _ in 0..n_edges {
        for _ in 0..n_edges {
            movement_costs.push(u8::MAX);
        }
    }
    find_movement_costs(&mut movement_costs, &edges);

    // The highest preasure found with time_left as index:
    // TODO: Should probably be best per position.
    let mut best = [-1i32; 31];
    let mut opened = [0u64; 31];

    // The search space left to expand:
    let mut current = BinaryHeap::new();
    // Add the initial state:
    // * sum of released preassure so far,
    // * released valves,
    // * open valves (one bit per valve),
    // * time_left,
    // * the position we are at.
    current.push((
        0i32,
        0i32,
        0u64,
        30u8,
        *name_mapping.get(&"AA".to_owned()).unwrap(),
    ));

    while let Some((sum, released, open, time_left, from)) = current.pop() {
        // If we don't open any more valves, what's the total released pressure:
        let minimum_total = sum + released * time_left as i32;

        // Store if this is currently the most total released pressure.
        if best[time_left as usize] < minimum_total {
            best[time_left as usize] = minimum_total;
            opened[time_left as usize] = open;
        }

        // Since the pressure is released after the minute, there's no need to open more.
        if time_left > 1 {
            for to in 0..n_edges {
                let valve = 1 << to;
                // The destination's valve is not opened yet:
                if open & valve == 0 {
                    let pressure = *pressure_for_valve[&(to as u8)];
                    // What's the cost of going there and opening the valve?
                    let movement_cost = movement_costs[from as usize * n_edges + to] + 1;

                    // Don't go if we can't make it in time.
                    if movement_cost < time_left {
                        let new_sum = sum + released * movement_cost as i32;
                        let new_released = released + pressure;
                        let minimum_total =
                            new_sum + new_released * (time_left - movement_cost) as i32;

                        // Don't go if there is already a better plan:
                        if best[(time_left - movement_cost) as usize] < minimum_total {
                            current.push((
                                new_sum,
                                new_released,
                                open | valve,
                                time_left - movement_cost,
                                to as u8,
                            ));
                        }
                    }
                }
            }
        }
    }

    /*
    for i in 0..best.len() {
        println!("{i:<2}: {:<5} {:10b}", best[i], opened[i]);
    }

    for from in 0 .. n_edges {
        print!("{from:<2}");
        for to in 0 .. n_edges {
            print!("{:3}", movement_costs[(from as usize) * n_edges + to as usize]);
        }
        println!();
    }
    dbg!(&pressure_for_valve);
    */

    *best.iter().max().unwrap()
}

#[aoc(day16, part2)]
pub fn solve_part2(_data: &[InputType]) -> SolutionType {
    // TODO
    /*
    // Translate strings to numbers:
    let mut name_mapping = HashMap::new();
    for (i, name) in data.iter().map(|(name, _, _)| name).enumerate() {
        name_mapping.insert(name, i as u8);
    }
    let name_mapping = name_mapping;

    // Translate edges to be from u8 to Vec<u8>:
    let mut edges = HashMap::new();
    let pressure_for_valve: HashMap<u8, _> = data
        .iter()
        .map(|(name, cost, valves)| {
            let from = *name_mapping.get(name).unwrap();
            edges.insert(
                from,
                valves
                    .iter()
                    .map(|valve| *name_mapping.get(valve).unwrap())
                    .collect::<Vec<_>>(),
            );
            (from, cost)
        })
        .collect();
    let edges = edges;

    let n_edges = edges.len();

    // Find the smallest cost to move from one valve to another:
    let mut movement_costs = Vec::with_capacity(n_edges * n_edges);
    for _ in 0..n_edges {
        for _ in 0..n_edges {
            movement_costs.push(u8::MAX);
        }
    }
    find_movement_costs(&mut movement_costs, &edges);

    // The highest preasure found with time_left as index:
    // TODO: Should probably be best per position.
    let mut best = [-1i32; 31];
    let mut opened = [0u64; 31];

    // The search space left to expand:
    let mut current = BinaryHeap::new();

    #[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
    struct Move {
        sum: i32,
        released_pressure: i32,
        extra_released_after_slow: i32,
        open_valves: u64,
        time_left: u8,
        extra_time_for_slow: u8,
        position: u8,
        position_for_slower: u8,
    }

    current.push(Move {
        sum: 0,
        released_pressure: 0,
        extra_released_after_slow: 0,
        open_valves: 0,
        time_left: 26,
        extra_time_for_slow: 0,
        position: *name_mapping.get(&"AA".to_owned()).unwrap(),
        position_for_slower: *name_mapping.get(&"AA".to_owned()).unwrap(),
    });

    /*
    while let Some(mov) = current.pop() {
        // If we don't open any more valves, what's the total released pressure:
        let minimum_total = mov.sum + mov.released * mov.time_left as i32;

        // Store if this is currently the most total released pressure.
        if best[mov.time_left as usize] < minimum_total {
            best[mov.time_left as usize] = minimum_total;
            opened[mov.time_left as usize] = mov.open_valves;
        }

        // Since the pressure is released after the minute, there's no need to open more.
        if mov.time_left > 1 {
            for to in 0..n_edges {
                let valve = 1 << to;
                // The destination's valve is not opened yet:
                if open & valve == 0 {
                    let pressure = *pressure_for_valve[&(to as u8)];
                    // What's the cost of going there and opening the valve?
                    let movement_cost = movement_costs[from as usize * n_edges + to] + 1;

                    // Don't go if we can't make it in time.
                    if movement_cost < time_left {
                        let new_sum = sum + released * movement_cost as i32;
                        let new_released = released + pressure;
                        let minimum_total =
                            new_sum + new_released * (time_left - movement_cost) as i32;

                        // Don't go if there is already a better plan:
                        if best[(time_left - movement_cost) as usize] < minimum_total {
                            current.push((
                                new_sum,
                                new_released,
                                open | valve,
                                time_left - movement_cost,
                                to as u8,
                            ));
                        }
                    }
                }
            }
        }
    }
    */

    /*
    for i in 0..best.len() {
        println!("{i:<2}: {:<5} {:10b}", best[i], opened[i]);
    }

    for from in 0 .. n_edges {
        print!("{from:<2}");
        for to in 0 .. n_edges {
            print!("{:3}", movement_costs[(from as usize) * n_edges + to as usize]);
        }
        println!();
    }
    dbg!(&pressure_for_valve);
    */

    *best.iter().max().unwrap()
    */
    0
}
