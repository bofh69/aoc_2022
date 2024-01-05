// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use advent_of_tools::*;

use std::collections::HashMap;
use std::collections::BinaryHeap;
// use itertools::*;

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

fn max_flow2(
    cache: &mut HashMap<(i32, i32, u64, u8, i32, u8, i32), SolutionType>,
    graph: &BiGraph<i32, (), u8, i32>,
    time_left: i32,
    flow: i32,
    open_doors: u64,
    my_pos: u8,
    my_delay: i32,
    elephant_pos: u8,
    elephant_delay: i32,
) -> SolutionType {
    let min_delay = my_delay.min(elephant_delay);
    let my_delay = my_delay - min_delay;
    let elephant_delay = elephant_delay - min_delay;
    let time_left = time_left - min_delay;

    if time_left <= 0 {
        return time_left * flow;
    }

    if let Some(value) = cache.get(&(
        time_left,
        flow,
        open_doors,
        my_pos,
        my_delay,
        elephant_pos,
        elephant_delay,
    )) {
        return *value;
    }

    let mut max = flow * (time_left * min_delay);

    if my_delay == 0 {
        if elephant_delay == 0 {
            if (elephant_pos != my_pos)
                && (graph.vertices[my_pos as usize].1 != 0)
                && (graph.vertices[elephant_pos as usize].1 != 0)
                && (open_doors & (1 << my_pos) == 0)
                && (open_doors & (1 << elephant_pos) == 0)
            {
                // Both open doors
                let new_flow = flow
                    + graph.vertices[my_pos as usize].1
                    + graph.vertices[elephant_pos as usize].1;
                let open_doors = open_doors | (1 << my_pos) | (1 << elephant_pos);

                max = max.max(
                    flow * (min_delay + 1)
                        + max_flow2(
                            cache,
                            graph,
                            time_left - 1,
                            new_flow,
                            open_doors,
                            my_pos,
                            0,
                            elephant_pos,
                            0,
                        ),
                );
            }

            // Both can move
            for (my_pos, my_delay, _) in graph
                .edges
                .get(&u8::try_from(my_pos).unwrap())
                .expect("my_pos")
            {
                if (graph.vertices[elephant_pos as usize].1 != 0)
                    && (open_doors & (1 << elephant_pos) == 0)
                {
                    // I move, the elephant opens the door:

                    let new_flow = flow + graph.vertices[elephant_pos as usize].1;
                    let open_doors = open_doors | (1 << elephant_pos);
                    max = max.max(
                        flow * min_delay - graph.vertices[elephant_pos as usize].1
                            + max_flow2(
                                cache,
                                graph,
                                time_left,
                                new_flow,
                                open_doors,
                                *my_pos,
                                *my_delay,
                                elephant_pos,
                                1,
                            ),
                    );
                }

                for (elephant_pos, elephant_delay, _) in graph
                    .edges
                    .get(&u8::try_from(elephant_pos).unwrap())
                    .expect("my_pos")
                {
                    // Both moves
                    max = max.max(
                        flow * min_delay
                            + max_flow2(
                                cache,
                                graph,
                                time_left,
                                flow,
                                open_doors,
                                *my_pos,
                                *my_delay,
                                *elephant_pos,
                                *elephant_delay,
                            ),
                    );
                }
            }
            if (graph.vertices[my_pos as usize].1 != 0) && (open_doors & (1 << my_pos) == 0) {
                let new_flow = flow + graph.vertices[my_pos as usize].1;
                let open_doors = open_doors | (1 << my_pos);

                for (elephant_pos, elephant_delay, _) in graph
                    .edges
                    .get(&u8::try_from(elephant_pos).unwrap())
                    .expect("my_pos")
                {
                    // Elephant moves, I open a door
                    max = max.max(
                        flow * min_delay - graph.vertices[my_pos as usize].1
                            + max_flow2(
                                cache,
                                graph,
                                time_left,
                                new_flow,
                                open_doors,
                                my_pos,
                                1,
                                *elephant_pos,
                                *elephant_delay,
                            ),
                    );
                }
            }
        } else {
            // Only I can move

            for (my_pos, my_delay, _) in graph
                .edges
                .get(&u8::try_from(my_pos).unwrap())
                .expect("my_pos")
            {
                // I move, elephant waits
                max = max.max(
                    flow * min_delay
                        + max_flow2(
                            cache,
                            graph,
                            time_left,
                            flow,
                            open_doors,
                            *my_pos,
                            *my_delay,
                            elephant_pos,
                            elephant_delay,
                        ),
                );
            }

            if (graph.vertices[my_pos as usize].1 != 0) && (open_doors & (1 << my_pos) == 0) {
                // I open a door
                let new_flow = flow + graph.vertices[my_pos as usize].1;
                let open_doors = open_doors | (1 << my_pos);

                // Elephant waits, I open a door
                max = max.max(
                    flow * min_delay - graph.vertices[my_pos as usize].1
                        + max_flow2(
                            cache,
                            graph,
                            time_left,
                            new_flow,
                            open_doors,
                            my_pos,
                            1,
                            elephant_pos,
                            elephant_delay,
                        ),
                );
            }
        }
    } else {
        // Only elephant can move

        for (elephant_pos, elephant_delay, _) in graph
            .edges
            .get(&u8::try_from(elephant_pos).unwrap())
            .expect("my_pos")
        {
            // Elephant moves, I wait
            max = max.max(
                flow * min_delay
                    + max_flow2(
                        cache,
                        graph,
                        time_left,
                        flow,
                        open_doors,
                        my_pos,
                        my_delay,
                        *elephant_pos,
                        *elephant_delay,
                    ),
            );
        }

        if (graph.vertices[elephant_pos as usize].1 != 0) && (open_doors & (1 << elephant_pos) == 0)
        {
            // Elephant open a door
            let new_flow = flow + graph.vertices[elephant_pos as usize].1;
            let open_doors = open_doors | (1 << elephant_pos);

            // Elephant open a door, I wait
            max = max.max(
                flow * min_delay - graph.vertices[elephant_pos as usize].1
                    + max_flow2(
                        cache,
                        graph,
                        time_left,
                        new_flow,
                        open_doors,
                        my_pos,
                        my_delay,
                        elephant_pos,
                        1,
                    ),
            );
        }
    }
    cache.insert(
        (
            time_left,
            flow,
            open_doors,
            my_pos,
            my_delay,
            elephant_pos,
            elephant_delay,
        ),
        max,
    );
    max
}

fn max_flow(
    cache: &mut HashMap<(i32, i32, u64, u8, u8), SolutionType>,
    graph: &UniGraph<i32, (), u8, i32>,
    time_left: i32,
    flow: i32,
    open_doors: u64,
    my_pos: u8,
    elephant_pos: u8,
) -> SolutionType {
    if time_left == 0 {
        return flow;
    }

    if let Some(value) = cache.get(&(time_left, flow, open_doors, my_pos, elephant_pos)) {
        return *value;
    }

    let mut max = flow * time_left;

    if (elephant_pos != my_pos)
        && (time_left >= 2)
        && (graph.vertices[my_pos as usize].1 != 0)
        && (graph.vertices[elephant_pos as usize].1 != 0)
        && (open_doors & (1 << my_pos) == 0)
        && (open_doors & (1 << elephant_pos) == 0)
    {
        // Both open doors
        let new_flow =
            flow + graph.vertices[my_pos as usize].1 + graph.vertices[elephant_pos as usize].1;
        let open_doors = open_doors | (1 << my_pos) | (1 << elephant_pos);

        max = max.max(
            flow * 2
                + max_flow(
                    cache,
                    graph,
                    time_left - 2,
                    new_flow,
                    open_doors,
                    my_pos,
                    elephant_pos,
                ),
        );
    }

    for (my_pos, _my_delay, _) in graph
        .edges
        .get(&u8::try_from(my_pos).unwrap())
        .expect("my_pos")
    {
        if (graph.vertices[elephant_pos as usize].1 != 0) && (open_doors & (1 << elephant_pos) == 0)
        {
            // I move, the elephant opens the door:

            let new_flow = flow + graph.vertices[elephant_pos as usize].1;
            let open_doors = open_doors | (1 << elephant_pos);
            max = max.max(
                flow - graph.vertices[elephant_pos as usize].1
                    + max_flow(
                        cache,
                        graph,
                        time_left - 1,
                        new_flow,
                        open_doors,
                        *my_pos,
                        elephant_pos,
                    ),
            );
        }

        for (elephant_pos, _elephant_delay, _) in graph
            .edges
            .get(&u8::try_from(elephant_pos).unwrap())
            .expect("my_pos")
        {
            // Both moves
            max = max.max(
                flow + max_flow(
                    cache,
                    graph,
                    time_left - 1,
                    flow,
                    open_doors,
                    *my_pos,
                    *elephant_pos,
                ),
            );
        }
    }

    if (graph.vertices[my_pos as usize].1 != 0) && (open_doors & (1 << my_pos) == 0) {
        let new_flow = flow + graph.vertices[my_pos as usize].1;
        let open_doors = open_doors | (1 << my_pos);

        for (elephant_pos, _elephant_delay, _) in graph
            .edges
            .get(&u8::try_from(elephant_pos).unwrap())
            .expect("my_pos")
        {
            // Elephant moves, I open a door
            max = max.max(
                flow - graph.vertices[my_pos as usize].1
                    + max_flow(
                        cache,
                        graph,
                        time_left - 1,
                        new_flow,
                        open_doors,
                        my_pos,
                        *elephant_pos,
                    ),
            );
        }
    }

    cache.insert((time_left, flow, open_doors, my_pos, elephant_pos), max);
    max
}

#[aoc(day16, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let data: Vec<_> = data
        .iter()
        .map(|(v, nd, e)| {
            (
                v.as_str(),
                *nd,
                e.iter().map(|e| (e.as_str(), ())).collect(),
            )
        })
        .collect();
    let graph: UniGraph<i32, (), u8, i32> = advent_of_tools::UniGraph::new(|_, _| 1, &data);

    assert!(graph.vertices.len() <= usize::try_from(u64::BITS).expect("u64 fits in usize"));

    let graph2 = BiGraph::compress(
        graph,
        |remove_node, node1, node2, _edge1, c1, _edge2, c2| {
            if *remove_node == 0 && (*node1 != 0 || *node2 != 0) {
                Some((c1 + c2, ()))
            } else {
                None
            }
        },
    );

    let start = graph2
        .vertices
        .iter()
        .enumerate()
        .find(|(_n, v)| v.0 == "AA")
        .expect("Vertex AA")
        .0 as u8;

    if let Some((start_e, _)) = graph2
        .vertices
        .iter()
        .enumerate()
        .find(|(_n, v)| v.0 == "ZZ")
    {
        println!("Simulate part 1");
        max_flow2(
            &mut HashMap::new(),
            &graph2,
            31,
            0,
            0,
            start,
            0,
            start_e as u8,
            0,
        )
    } else {
        max_flow2(&mut HashMap::new(), &graph2, 27, 0, 0, start, 0, start, 0)
    }

    /*
    // Test with elephan in special node, to get same answer as part 1:
    let start3 = graph
        .vertices
        .iter()
        .enumerate()
        .find(|(_n, v)| v.0 == "ZZ")
        .expect("Vertex ZZ")
        .0 as u8;
    max_flow(&mut HashMap::new(), &graph, 30, 0, 0, start, start3)

    */

    /*
    let start = graph
        .vertices
        .iter()
        .enumerate()
        .find(|(_n, v)| v.0 == "AA")
        .expect("Vertex AA")
        .0 as u8;

    // max_flow(&mut HashMap::new(), &graph, 26, 0, 0, start, 0, start, 0)
    max_flow(&mut HashMap::new(), &graph, 26, 0, 0, start, start)
        */

    /*

    let mut frontier = Vec::new();
    let mut visited = std::collections::HashSet::new();

    // time left, flow, my position, elephant's pos.
    frontier.push((27i32, 0i32, 0i32, 0u64, start, 1, start, 1, vec![]));
    let mut most_flow = 0;
    while let Some((
        time_left,
        flow,
        ack_flow,
        opened,
        my_pos,
        my_delay,
        elephant_pos,
        elephant_delay,
        mut trace,
    )) = frontier.pop()
    {
        if time_left == i32::MAX {
            visited.remove(&(opened, my_pos, elephant_pos));
            continue;
        }

        if !visited.insert((opened, my_pos, elephant_pos)) {
            continue;
        }

        let mut delay = 1.max(my_delay.min(elephant_delay));
        if delay > time_left {
            delay = time_left;
        }

        let time_left = time_left - delay;
        let ack_flow = ack_flow + flow * delay;

        if time_left == 0 {
            if ack_flow > most_flow {
                println!(
                    "New ack_flow={}, flow={}, opened={}:\n    {:?}",
                    ack_flow,
                    flow,
                    opened.count_ones(),
                    trace
                );
            }
            most_flow = most_flow.max(ack_flow);
            continue;
        }

        frontier.push((i32::MAX, 0, 0, opened, my_pos, 0, elephant_pos, 0, vec![]));

        let my_delay = 0.max(my_delay - delay);
        let elephant_delay = 0.max(elephant_delay - delay);

        trace.push(format!(
            "{time_left} {my_delay} {elephant_delay} {flow} {ack_flow} {0} {1}",
            graph.vertices[my_pos as usize].0, graph.vertices[elephant_pos as usize].0
        ));

        if my_delay == 0 && elephant_delay == 0 {
            if (my_pos != elephant_pos)
                && (graph.vertices[my_pos as usize].1 != 0)
                && (graph.vertices[elephant_pos as usize].1 != 0)
                && (1 << elephant_pos) & opened == 0
                && (1 << my_pos) & opened == 0
            {
                // Both open valves
                let opened = opened | (1 << elephant_pos) | (1 << my_pos);
                let flow = flow
                    + graph.vertices[elephant_pos as usize].1
                    + graph.vertices[my_pos as usize].1;
                let ack_flow = ack_flow
                    - (graph.vertices[elephant_pos as usize].1 + graph.vertices[my_pos as usize].1);
                frontier.push((
                    time_left,
                    flow,
                    ack_flow,
                    opened,
                    my_pos,
                    1,
                    elephant_pos,
                    1,
                    trace.clone(),
                ));
            }
        }

        if my_delay == 0 {
            for (my_pos, my_delay, _) in graph
                .edges
                .get(&u8::try_from(my_pos).unwrap())
                .expect("my_pos")
            {
                if elephant_delay == 0 {
                    if ((1 << elephant_pos) & opened == 0)
                    && (graph.vertices[elephant_pos as usize].1 != 0) {
                        // Elephant opens the valve, I move
                        let opened = opened | (1 << elephant_pos);
                        let flow = flow + graph.vertices[elephant_pos as usize].1;
                        let ack_flow = ack_flow - graph.vertices[elephant_pos as usize].1;
                        let mut trace = trace.clone();
                        trace.push("Elephant opens valve".to_string());
                        frontier.push((
                            time_left,
                            flow,
                            ack_flow,
                            opened,
                            *my_pos,
                            *my_delay,
                            elephant_pos,
                            1,
                            trace,
                        ));
                    }

                    for (elephant_pos, elephant_delay, _) in
                        graph.edges.get(&elephant_pos).expect("elephant_pos")
                    {
                        let mut trace = trace.clone();
                        trace.push("Both moves".to_string());
                        // We both move
                        frontier.push((
                            time_left,
                            flow,
                            ack_flow,
                            opened,
                            *my_pos,
                            *my_delay,
                            *elephant_pos,
                            *elephant_delay,
                            trace,
                        ));
                    }
                } else {
                    let mut trace = trace.clone();
                    trace.push("Elephant waits".to_string());
                    // Elephant waits more, I move
                    frontier.push((
                        time_left,
                        flow,
                        ack_flow,
                        opened,
                        *my_pos,
                        *my_delay,
                        elephant_pos,
                        elephant_delay,
                        trace,
                    ));
                }
            }
            if ((1 << my_pos) & opened == 0)
                && (graph.vertices[my_pos as usize].1 != 0) {
                let opened = opened | (1 << my_pos);
                let flow = flow + graph.vertices[my_pos as usize].1;
                let ack_flow = ack_flow - graph.vertices[my_pos as usize].1;
                if elephant_delay == 0 {
                    for (elephant_pos, _, _) in graph
                        .edges
                        .get(&u8::try_from(elephant_pos).unwrap())
                        .expect("elephant_pos")
                    {
                    let mut trace = trace.clone();
                    trace.push("I open, elephant moves".to_string());
                        // I open the valve, the elephant moves
                        frontier.push((
                            time_left,
                            flow,
                            ack_flow,
                            opened,
                            my_pos,
                            1,
                            *elephant_pos,
                            elephant_delay,
                            trace,
                        ));
                    }
                }
            }
        } else {
            if ((1 << elephant_pos) & opened == 0)
                && (graph.vertices[elephant_pos as usize].1 != 0) {
                // Elephant opens the valve, I wait
                let opened = opened | (1 << elephant_pos);
                let flow = flow + graph.vertices[elephant_pos as usize].1;
                let ack_flow = ack_flow - graph.vertices[elephant_pos as usize].1;
                let mut trace = trace.clone();
                trace.push("Elephant opens valve, I wait".to_string());
                frontier.push((
                    time_left,
                    flow,
                    ack_flow,
                    opened,
                    my_pos,
                    my_delay,
                    elephant_pos,
                    1,
                    trace,
                ));
            }
            for (elephant_pos, elephant_delay, _) in
                graph.edges.get(&elephant_pos).expect("elephant_pos")
            {
                let mut trace = trace.clone();
                trace.push("I wait, elephant moves".to_string());
                // Elephant moves, I wait more
                frontier.push((
                    time_left,
                    flow,
                    ack_flow,
                    opened,
                    my_pos,
                    my_delay,
                    *elephant_pos,
                    *elephant_delay,
                    trace.clone(),
                ));
            }
        }
    }

    most_flow
        */
}
