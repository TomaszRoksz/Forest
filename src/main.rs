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

    let mut visualization: bool = false;
    let mut find_optimal_afforestation: bool = false;
    let mut examine_average_burning: i32 = 1;

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
            "-e" => {
                if i + 1 < args.len() {
                    examine_average_burning = args[i + 1].parse::<i32>().unwrap_or(x);
                    i += 1;
                }            }
            "-v" => visualization = true,
            
            "-f" => find_optimal_afforestation = true,
            _ => {}
        }
        i += 1;
    }

    if visualization {
        if find_optimal_afforestation{
            eprintln!("Error: -f cannot be combined with -v");
            std::process::exit(1);
        }

        if examine_average_burning != 1 {
            eprintln!("Error: -e cannot be combined with -v");
            std::process::exit(1);
        }
    }

    if examine_average_burning < 1 {
        eprintln!("Error: -e must be greater than 0");
        std::process::exit(1);
    }

    if find_optimal_afforestation {
        find_optimal_afforestation_function(x, y, examine_average_burning);
        return;
    }

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

    let forest_size = x * y;
    let to_plant = (afforestation * forest_size as f32 * 0.01) as usize;
    let mut forest = Forest::new(x, y);
    let mut afforestation_after_burning_list: Vec<f32> = Vec::new();

    for _ in 0..examine_average_burning {
        let after = find_afforestation(
            &mut forest,
            x,
            forest_size,
            to_plant,
            afforestation,
        );
        afforestation_after_burning_list.push(after);
    }


    let average_afforestation_after_burning: f32 = 
        afforestation_after_burning_list.iter().sum::<f32>() / afforestation_after_burning_list.len() as f32;

    println!("Percentage of unburned trees: {:.2}%", average_afforestation_after_burning);

    if visualization {
        visualize(&forest, x as u32, y as u32);
    }
}

fn find_afforestation(
    forest: &mut Forest,
    x: i32,
    forest_size: i32,
    to_plant: usize,
    afforestation: f32,
) -> f32 {
    let mut rng = thread_rng();
    forest.clear();

    let mut positions: Vec<i32> = (0..forest_size).collect();
    positions.shuffle(&mut rng);
    for &pos in positions.iter().take(to_plant) {
        forest.add_tree(Point::new(pos));
    }

    let secret_x = rng.gen_range(0..forest_size);
    forest.burn_tree(Point::new(secret_x), x);

    forest.get_trees_percentage(afforestation)
}


fn find_optimal_afforestation_function(x: i32, y: i32, examine_runs: i32) {
    let total = (x * y) as usize;

    let mut best_p = 0;
    let mut best_survival_afforestation = -1.0f32;

    let mut forest = Forest::new(x, y);

    for p in 1..=99 {
        let mut sum = 0.0;
        let to_plant = ((p as f32 / 100.0) * total as f32).round() as usize;

        for _ in 0..examine_runs {

            sum += find_afforestation(
                &mut forest,
                x,
                total as i32,
                to_plant,
                p as f32,
            );
        }

        let avg = sum / examine_runs as f32;
        if avg > best_survival_afforestation {
            best_survival_afforestation = avg;
            best_p = p;
        }
    }

    println!(
        "Optimal afforestation: {}% → average remaining trees: {:.2}%",
        best_p, best_survival_afforestation
    );
}