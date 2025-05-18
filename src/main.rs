use rand::prelude::*;   // for thread_rng() and SliceRandom
use std::io;

mod point;
mod forest;
mod tree_state;
mod visualize;

use point::Point;
use forest::Forest;
use tree_state::TreeState;
use visualize::visualize;

fn main() {
    let afforestation: u32 = loop {
        println!("Please input percent of afforestation (0–100):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num @ 0..=7500) => break num,
            _ => {
                println!("Please input a valid number between 0 and 100.");
                continue;
            }
        }
    };

    let empty_state = TreeState::new('e');
    let mut forest = Forest::new();
    for x in 0..150 {
        for y in 0..50 {
            let p = Point::new(x, y);
            forest.add_tree(p, Some(empty_state));
        }
    }

    let total_cells = forest.get_trees().len();
    let to_plant = (afforestation as usize * total_cells) / 100;

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

    let secret_x=rand::thread_rng().gen_range(0..=149);
    let secret_y=rand::thread_rng().gen_range(0..=49);

    forest.burn_tree(Point::new(secret_x, secret_y));

    visualize(&forest, 150, 49);
}
