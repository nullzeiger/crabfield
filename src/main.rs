// Copyright (c) 2024 Ivan Guerreschi. All rights reserved.
// Licensed under the MIT License.
// See LICENSE in the project root for license information.

use rand::Rng;
use std::io;

// minefield with a size 10x10
const ROWSCOLS: usize = 10;
// Number of mines
const MINES: usize = 5;
// Map symbol
const MAPSYMBOL: char = '*';

// Arrive data structure
#[derive(Debug, Copy, Clone)]
struct Arrive {
    symbol: char,
    y: i32,
    x: i32,
}

// Hero data structure
#[derive(Debug, Copy, Clone)]
struct Hero {
    symbol: char,
    y: i32,
    x: i32,
}

// Mine data structure
#[derive(Debug, Copy, Clone)]
struct Mine {
    symbol: char,
    y: i32,
    x: i32,
}

// Create random arrive
impl Arrive {
    fn new() -> Arrive {
        let mut rng = rand::thread_rng();

        Arrive {
            symbol: 'P',
            y: rng.gen_range(0..10),
            x: rng.gen_range(0..10),
        }
    }
}

// Create hero
impl Hero {
    fn new(symbol: char, y: i32, x: i32) -> Hero {
        Hero { symbol, y, x }
    }
}

// Create 5 random mine
impl Mine {
    fn new(symbol: char, y: i32, x: i32) -> Mine {
        Mine { symbol, y, x }
    }

    fn create_mines() -> Vec<Mine> {
        let mut rng = rand::thread_rng();
        let mut mines = Vec::with_capacity(MINES);

        for _ in 0..MINES {
            mines.push(Mine::new('X', rng.gen_range(0..10), rng.gen_range(0..10)));
        }

        mines
    }
}

// Create map
fn create_map(arrive: Arrive) -> [[char; ROWSCOLS]; ROWSCOLS] {
    let mut map: [[char; ROWSCOLS]; ROWSCOLS] = [[' '; 10]; 10];

    for i in 0..ROWSCOLS {
        for j in 0..ROWSCOLS {
            map[i][j] = MAPSYMBOL;
        }
    }

    map[arrive.y as usize][arrive.x as usize] = arrive.symbol;

    map
}

// Clear console and print map
fn print_map(mut map: [[char; ROWSCOLS]; ROWSCOLS], hero: &Hero) {
    print!("\x1B[2J\x1B[H");

    println!("Command to move(hjkl) and exit(q)");

    map[hero.y as usize][hero.x as usize] = hero.symbol;
    for row in map.iter() {
        for element in row.iter() {
            print!("{}", element);
        }

        println!();
    }
}

fn main() {
    let arrive = Arrive::new();
    let mut hero = Hero::new('@', 0, 0);
    let mines = Mine::create_mines();
    let map = create_map(arrive);

    print_map(map, &hero);

    let mut game_loop = true;

    while game_loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match command.as_str().trim() {
            "q" => break,
            "l" => {
                hero.x += 1;
                if hero.x > 9 {
                    hero.x = 9;
                }
                print_map(map, &hero);
            }

            "h" => {
                hero.x -= 1;
                if hero.x < 0 {
                    hero.x = 0;
                }
                print_map(map, &hero);
            }

            "k" => {
                hero.y += 1;
                if hero.y > 9 {
                    hero.y = 9;
                }
                print_map(map, &hero);
            }

            "j" => {
                hero.y -= 1;
                if hero.y < 0 {
                    hero.y = 0;
                }
                print_map(map, &hero);
            }
            _ => print_map(map, &hero),
        }

        if hero.x == arrive.x && hero.y == arrive.y {
            println!("Win");
            break;
        }

        for mine in mines.iter() {
            if hero.x == mine.x && hero.y == mine.y {
                hero.symbol = mine.symbol;
                print_map(map, &hero);
                println!("Game Over!!");
                game_loop = false;
                break;
            }
        }
    }
}
