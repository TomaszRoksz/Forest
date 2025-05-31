use rand::prelude::*;  
use std::io;
use std::env;

mod point;
mod forest;
mod tree_state;
mod visualize;

use point::Point;
use forest::Forest;
use tree_state::TreeState;
use visualize::visualize;

fn main() {
    let afforestation: f32 = loop {
        println!("Please input percent of afforestation (0–100):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num @ 0.00f32..=100.00f32) => break num,
            _ => {
                println!("Please input a valid number between 0 and 100.");
                continue;
            }
        }
    };


    let empty_state = TreeState::new('e');
    let mut forest = Forest::new();

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


    for x_coordinate in 0..x {
        for y_coordinate in 0..y {
            let p = Point::new(x_coordinate, y_coordinate);
            forest.add_tree(p, Some(empty_state));
        }
    }

    let total_cells = forest.get_trees().len();
    let to_plant = (afforestation * total_cells as f32 * 0.01) as usize;

    let mut rng = thread_rng();
    let mut positions: Vec<Point> = forest
        .get_trees()
        .iter()
        .map(|t| t.location)
        .collect();

    positions.shuffle(&mut rng);

    for p in positions.into_iter().take(to_plant) {
        forest.plant_tree(p);
    }

    let secret_x=rand::thread_rng().gen_range(0..=x-1);
    let secret_y=rand::thread_rng().gen_range(0..=y-1);

    forest.burn_tree(Point::new(secret_x, secret_y));

    forest.print_trees_percentage();

    visualize(&forest, x as u32, y as u32);
}
