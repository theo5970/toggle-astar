use crate::toggle::{core::ButtonFunction, utils::BitArray};
use priority_queue::PriorityQueue;
use std::{collections::HashMap, hash::Hash};
use stopwatch::Stopwatch;

use super::core::{Coordinate, Grid, ToggleLevel};

#[derive(PartialEq, Eq, Hash, Clone)]
struct SolverItem {
    state: BitArray,
    coord: Coordinate,
    orders: Vec<Coordinate>,
}

impl SolverItem {
    fn new() -> SolverItem {
        SolverItem {
            state: BitArray::new(0),
            coord: Coordinate::new(0, 0),
            orders: Vec::new(),
        }
    }
}

pub enum Result {
    Success(Vec<Coordinate>),
    Fail,
}

fn calculate_diff(b1: &BitArray, b2: &BitArray) -> i32 {
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

    diff * diff + 2 * a.orders.len() as i32
}

fn is_arrow(a: ButtonFunction) -> bool {
    match a {
        ButtonFunction::OneArrow(_) | ButtonFunction::TwoArrow(_) | ButtonFunction::FourArrow => {
            true
        }
        _ => false,
    }
}

pub fn run_astar(level: &ToggleLevel) -> Result {
    let mut result = Result::Fail;

    let mut grid = Grid::from_level(&level);
    let mut previous_state = grid.get_states();

    grid.set_all_state(false);

    let target_state = grid.get_states();

    let mut visited = HashMap::new();
    visited.insert(previous_state.to_base64(), true);

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

            let cost = calculate_cost(&item, &target_state);
            pq.push(item, -1 * cost);

            visited.insert(grid.get_states().to_base64(), true);
        }
    }

    let stopwatch = Stopwatch::start_new();
    let mut iterations = 0;
    while !pq.is_empty() {
        iterations += 1;

        let item = pq.pop().unwrap().0;
        let item_func = grid.at(item.coord.x, item.coord.y).unwrap().func;

        let diff = calculate_diff(&item.state, &target_state);
        if iterations % 10000 == 0 {
            println!(
                "?{},{},{},{}",
                iterations,
                diff,
                pq.len(),
                item.orders.len()
            );
        }

        if diff == 0 {
            println!(
                "!{},{},{}",
                iterations,
                item.orders.len(),
                stopwatch.elapsed_ms()
            );
            result = Result::Success(item.orders);
            break;
        }

        previous_state = item.state;
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let mut should_skip: bool = false;

                match grid.at(x, y).unwrap().func {
                    ButtonFunction::Nothing => {
                        should_skip = true;
                    }
                    _ => {}
                }

                if x == item.coord.x && y == item.coord.y {
                    let func = grid.at(x, y).unwrap().func;

                    if is_arrow(item_func) && is_arrow(func) {
                        should_skip = true;
                    }
                }

                if should_skip {
                    continue;
                }

                grid.set_states(&previous_state);
                grid.click(x, y);

                let states_base64 = grid.get_states().to_base64();
                if visited.contains_key(&states_base64) == false {
                    let mut new_item = SolverItem::new();
                    new_item.orders.extend_from_slice(&item.orders[..]);
                    new_item.orders.push(Coordinate::new(x, y));
                    new_item.state = grid.get_states();

                    let cost = calculate_cost(&new_item, &target_state);
                    pq.push(new_item, -1 * cost);

                    visited.insert(states_base64, true);
                }
            }
        }
    }

    result
}
