use rand::prelude::*;
use std::io;
use std::env;

mod point;
mod forest;
mod tree_state;
mod visualize;

use point::Point;
use forest::Forest;
use visualize::visualize;

fn main() {
    let afforestation: f32 = loop {
        println!("Please input percent of afforestation (0–100):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(num @ 0.0..=100.0) => break num,
            _ => {
                println!("Please input a valid number between 0 and 100.");
                continue;
            }
        }
    };

    let mut x: i32 = 200;
    let mut y: i32 = 112;
    let args: Vec<String> = env::args().collect();
    let mut i: usize = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-x" => {
                if i + 1 < args.len() {
                    x = args[i + 1].parse::<i32>().unwrap_or(x);
                    i += 1;
                }
            }
            "-y" => {
                if i + 1 < args.len() {
                    y = args[i + 1].parse::<i32>().unwrap_or(y);
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let forest_size = x * y;
    let to_plant = (afforestation * forest_size as f32 * 0.01) as usize;
    let mut forest = Forest::new(x, y);
    let mut rng = thread_rng();

    let mut positions: Vec<i32> = (0..forest_size).collect();
    positions.shuffle(&mut rng);
    for &pos in positions.iter().take(to_plant) {
        forest.add_tree(Point::new(pos));
    }

    let secret_x = rng.gen_range(0..forest_size);
    forest.burn_tree(Point::new(secret_x), x);
    forest.print_trees_percentage();
    visualize(&forest, x as u32, y as u32);
}