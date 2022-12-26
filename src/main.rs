// Rust implementation of "the game of life".
// See https://en.wikipedia.org/wiki/Conway's_Game_of_Life
// for a description of what this is about.
//
// Usage: life -f <file> [-t ticks] [-a arena size]
//
// Author Emiliano Testa, etesta@undo.io

extern crate clap;
use clap::{Arg, App, ArgMatches};
use std::process;
use std::io::Error;
use spin_sleep;

use crate::matrix::Matrix;

const ARENA_SIZE: usize = 40;

pub mod point;
pub mod matrix;

fn update_screen(matrix: &Vec<Vec<bool>>, iter: u32) {
    print!("{}[2J", 27 as char);
	println!("Iteration {}", iter);
	println!("");
    for line in matrix {
        for cell in line {
            print!("{}", if *cell {1} else {0});
        }
        print!("\n");
    }
}

fn run(matches: ArgMatches) -> Result<(), Error> {

    let input_file = matches.value_of("input file").unwrap();
    let ticks_str = matches.value_of("ticks");
    let arena_str = matches.value_of("arena");
    let mut num_ticks = 1000;
    let mut arena_size = ARENA_SIZE;
    match ticks_str {
        None => {
            println!("Using default number of ticks of {}", num_ticks)
        },
        Some(s) => {
            match s.parse::<u32>() {
                Ok(n) => {
                    num_ticks = n;
                    println!("Set number of ticks to {}", num_ticks)
                },
                Err(_) => println!("Number of ticks defaulted to {} as {} is not a number", num_ticks, s),
            }
        }
    }
    match arena_str {
        None => {
            println!("Using default arena size of {}", arena_size)
        },
        Some(s) => {
            match s.parse::<u32>() {
                Ok(n) => {
                    arena_size = n as usize;
                    println!("Set arena size to {}", arena_size)
                },
                Err(_) => println!("Arena size defaulted to {} as {} is not a number", arena_size, s),
            }
        }
    }
    let mut matrix = Matrix::new(arena_size);

    matrix.parse_file(input_file)?;
	let spin_sleeper = spin_sleep::SpinSleeper::new(100_000);
    for ii in 0..num_ticks {
        let new_m = matrix.update_matrix();
        matrix = new_m;
    	update_screen(&matrix.m, ii);
		//matrix = new_matrix;
		spin_sleeper.sleep_ns(500_000_000);
    }
    Ok(())
}

fn main() {
    let matches = App::new("Game of life")
        .version("0.1.0")
        .author("Emiliano Testa <testa.emiliano@gmail.com>")
        .about("Play the game of life")
        .arg(Arg::with_name("input file")
                 .short("f")
                 .long("file")
                 .takes_value(true)
                 .required(true)
                 .help("The starting point of the game, contains a matrix of 1 and 0, 1 is populated, 0 is unpopulated"))
        .arg(Arg::with_name("ticks")
                 .short("t")
                 .long("ticks")
                 .takes_value(true)
                 .help("Number of generations you want to run the game for. Rate is 2 generations per second"))
        .arg(Arg::with_name("arena")
                 .short("a")
                 .long("arena")
                 .takes_value(true)
                 .help("Size of the arena you want to run the game in. The arena area will be this number squared"))
        .get_matches();

    if let Err(e) = run(matches) {
        println!("Error running life: {}", e);
        process::exit(1);
    }
}

// Unit tests

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//}
