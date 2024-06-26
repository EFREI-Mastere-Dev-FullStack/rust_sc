extern crate noise;
extern crate rand;
mod utils;
mod core;


use std::io;
use std::time::Duration;
use std::thread::sleep;

use rand::{Rng, thread_rng};
use crossterm::event::{self, Event, KeyCode};
use crate::core::game::Game;
use crate::core::robot::Robot;
use crate::utils::robot_type::Robot_type;

// debug main
fn main() {
    println!("Enter the map seed:");

    let mut input = String::new();
    let mut seed: u32 = 0;
    io::stdin().read_line(&mut input).expect("Error while reading the input");

     if !input.trim().is_empty() {
        seed = input.trim().parse().expect("Invalid input");
    } else {
        let mut rng = thread_rng();
        seed = rng.gen();
    }

    println!("Enter the map height:");
    let mut input = String::new();
    let mut height: usize = 0;
    io::stdin().read_line(&mut input).expect("Error while reading the input");

     if !input.trim().is_empty() {
        height = input.trim().parse().expect("Invalid input");
    } else {
        height = 40;
    }

    println!("Enter the map width:");
    input = String::new();
    let mut width: usize = 0;
    io::stdin().read_line(&mut input).expect("Error while reading the input");

    if !input.trim().is_empty() {
        width = input.trim().parse().expect("Invalid input");
    } else {
        width = 80;
    }

    println!("View with fog of war (Y) or all map (N):");
    input = String::new();
    let mut fow: char = ' ';
    io::stdin().read_line(&mut input).expect("Error while reading the input");

    if !input.trim().is_empty() {
        fow = input.trim().parse().expect("Invalid input");
    } else {
        fow = 'N';
    }

    println!("Generating map with seed: {}", seed);
    let mut game: Game = Game::new(width, height, seed);
    let robot: Robot = Robot::new(width / 2, height / 2, Robot_type::Scout, &mut game);
    game.add_robot(robot);
    let robot2: Robot = Robot::new(width / 2 + 1, height / 2, Robot_type::Harvester, &mut game);
    game.add_robot(robot2);
    let robot3: Robot = Robot::new(width / 2, height / 2 + 1, Robot_type::Scientist, &mut game);
    game.add_robot(robot3);

    game.update_known_maps();
    let mut paused: bool = false;
    let mut quit: bool = false;
    loop {
        if !paused {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Press 'p' + ENTER to pause, 'q' + ENTER to quit or 'r' + ENTER to restart with the same configuration");

            if fow == 'N' {
                game.print_map();
            } else {
                let data_treatment = game.data_treatment();
                let is_treating_data = game.is_treating_data();
                game.base.print_merged_map(&mut game.robots, &mut game.point, data_treatment, is_treating_data);
            }
            game.move_robots();
            game.update_known_maps();
            game.create_robot();
            game.treat_data();
            sleep(Duration::from_millis(5));
        }

        if event::poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char('p') => {
                        paused = !paused;
                        if paused {
                            println!("--- PAUSED ---\nPress 'p' + ENTER to pause, 'q' + ENTER to quit or 'r' + ENTER to restart with the same configuration");
                        }
                    }
                    KeyCode::Char('q') => {
                        quit = true;
                    }
                    KeyCode::Char('r') => {
                        game = Game::new(width, height, seed);
                        let robot: Robot = Robot::new(width / 2, height / 2, Robot_type::Scout, &mut game);
                        game.add_robot(robot);
                        let robot2: Robot = Robot::new(width / 2 + 1, height / 2, Robot_type::Harvester, &mut game);
                        game.add_robot(robot2);
                        let robot3: Robot = Robot::new(width / 2, height / 2 + 1, Robot_type::Scientist, &mut game);
                        game.add_robot(robot3);

                        game.update_known_maps();
                        paused = false;
                        quit = false;
                    }
                    _ => {}
                }
            }
        }

        if quit {
           break;
        }
    }
}