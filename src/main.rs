use std::io;

use toggle::astar;
use toggle::core::{Coordinate, Grid, ToggleLevel};
use toggle::reader;

mod toggle;

fn main() {
    /*let args: Vec<String> = env::args().collect();
    let code;
    if args.len() == 2 {
        code = args[1].clone();
    } else {
        panic!("Please give level code via argument!");
    }*/

    let mut code = String::new();
    io::stdin()
        .read_line(&mut code)
        .expect("Failed to read input.");

    let level = reader::parse(code);

    let result = astar::run_astar(&level);
    match result {
        astar::Result::Success(orders) => {
            calculate_difficulty(&level, orders);
        }
        astar::Result::Fail => {
            println!("Failed to solve!");
        }
    }
}

fn calculate_difficulty(level: &ToggleLevel, orders: Vec<Coordinate>) {
    let mut diff: Vec<u32> = Vec::new();

    let mut grid = Grid::from_level(level);
    let total_buttons = grid.width() * grid.height();
    for _ in 0..total_buttons {
        diff.push(0);
    }

    let mut previous_state = grid.get_states();

    println!("{} Clicks", orders.len());
    for ord in orders {
        grid.click(ord.x, ord.y);
        let current_state = grid.get_states();

        for i in 0..(total_buttons as usize) {
            if current_state.get(i) != previous_state.get(i) {
                diff[i] += 1;
            }
        }
        previous_state = current_state;
    }

    let mut mul: f64 = 1.0;
    for i in 0..diff.len() {
        if diff[i] != 0 {
            mul *= diff[i] as f64;
        }
    }
    mul = f64::powf(mul, 1.0 / total_buttons as f64);

    let difficulty = (mul - 1.0) * 10.0;
    println!("Difficulty: {}", difficulty);
}
