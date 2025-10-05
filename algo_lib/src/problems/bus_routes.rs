use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let res = 0;

        for (bus, routes) in routes.iter().enumerate() {
            for route in routes.iter() {
                graph.entry(*route).or_default().push(bus as i32);
            }
        }

        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        queue.push_back((source, 0));

        let mut visited_stops = HashSet::new();
        visited_stops.insert(source);

        let mut visited_buses = HashSet::new();

        while let Some((route, moves)) = queue.pop_front() {
            if route == target {
                return moves;
            }

            if let Some(buses) = graph.get(&route) {
                for bus in buses.iter() {
                    if visited_buses.contains(bus) {
                        continue;
                    }

                    visited_buses.insert(*bus);
                    for route in routes[*bus as usize].iter() {
                        if !visited_stops.contains(&route) {
                            visited_stops.insert(*route);
                            queue.push_back((*route, moves + 1));
                        }
                    }
                }
            }
        }

        -1
    }
}
