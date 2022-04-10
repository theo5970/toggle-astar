use crate::toggle::{utils::BitArray, core::ButtonFunction};
use priority_queue::PriorityQueue;
use stopwatch::Stopwatch;
use toggle::core::Coordinate;

mod toggle;

use std::{hash::{Hash}, collections::HashMap};
use std::io;

#[derive(PartialEq, Eq, Hash, Clone)]
struct SolverItem {
    state: BitArray,
    orders: Vec<Coordinate>,
    depth: i32,
}

impl SolverItem {
    fn new() -> SolverItem {
        SolverItem {
            state: BitArray::new(0),
            orders: Vec::new(),
            depth: 0,
        }
    }
}

fn calculate_diff(b1: &BitArray, b2: &BitArray) -> i32{
    let mut diff = 0;
    assert_eq!(b1.len(), b2.len());

    for i in 0..b2.len() {
        if b1.get(i) != b2.get(i) {
            diff += 1;
        }
    }

    diff
}

fn calculate_cost(a: &SolverItem, target_state: &BitArray) -> i32 {
    let solver_state = &a.state;
    let diff = calculate_diff(solver_state, target_state);

    diff * diff + a.depth
}

fn main() {
    let mut code = String::new();
    io::stdin()
        .read_line(&mut code)
        .expect("Failed to read input.");

    let level = toggle::reader::parse(code);

    let mut grid = toggle::core::Grid::from_level(&level);
    let mut previous_state = grid.get_states();
    grid.print();
    grid.set_all_state(false);

    let target_state = grid.get_states();

    let mut visited = HashMap::new();
    let mut pq = PriorityQueue::new();
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let mut is_nothing: bool = false;

            match grid.at(x, y).unwrap().func {
                ButtonFunction::Nothing => {
                    is_nothing = true;
                }
                _ => {}
            }

            if is_nothing {
                continue;
            }
            grid.set_states(&previous_state);
            grid.click(x, y);



            let mut item = SolverItem::new();
            item.orders.push(Coordinate::new(x, y));
            item.state = grid.get_states();
            item.depth = 1;

            let cost = calculate_cost(&item, &target_state);
            pq.push(item, -1 * cost);

            visited.insert(grid.get_states().to_base64(), true);
        }
    }

    let stopwatch = Stopwatch::start_new();
    
    let mut is_solved: bool = false;

    let mut iterations = 0;
    while !pq.is_empty() {
        iterations+= 1;

        let item = pq.pop().unwrap().0;
        let diff = calculate_diff(&item.state, &target_state);
        if iterations % 10000 == 0 {
            println!("Iterations: {} / Best: {}", iterations, diff);
        }

        if diff == 0 {
            is_solved = true;
            println!("Solved! with {} Iterations ({} ms)", iterations, stopwatch.elapsed_ms());
            println!("{} Clicks", item.orders.len());
            println!("{:?}", item.orders);
            
            grid.set_states(&item.state);
            grid.print();
            break;
        }

        previous_state = item.state;
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let mut is_nothing: bool = false;
    
                match grid.at(x, y).unwrap().func {
                    ButtonFunction::Nothing => {
                        is_nothing = true;
                    }
                    _ => {}
                }

                if is_nothing {
                    continue;
                }

                grid.set_states(&previous_state);
                grid.click(x, y);

                let states_base64 = grid.get_states().to_base64();
                if !visited.contains_key(&states_base64) {
                    let mut new_item = SolverItem::new();
                    new_item.orders.extend_from_slice(&item.orders[..]);
                    new_item.orders.push(Coordinate::new(x, y));
                    new_item.state = grid.get_states();
                    new_item.depth = item.depth + 1;

                    let cost = calculate_cost(&new_item, &target_state);
                    pq.push(new_item, -1 * cost);

                    visited.insert(states_base64, true);
                }
            }
        } 
    }

    if is_solved == false {
        println!("Failed to solve!");
    }
}
