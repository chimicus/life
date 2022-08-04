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
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use spin_sleep;
const ARENA_SIZE: usize = 40;

struct Point {
	row: usize,
	col: usize,
}

fn center_matrix(matrix: &mut Vec<Vec<bool>>, start: Point, end: Point) -> Result<(), Error> {
	let half_length = Point {
		row: (end.row - start.row) >> 1,
		col: (end.col - start.col) >> 1,
	};
	let matrix_len = matrix.capacity();
	let new_start = Point {
		row: matrix_len - half_length.row,
		col: matrix_len - half_length.col,
	};
	for line in &mut matrix.into_iter() {
		line.rotate_right(new_start.col);
	}
	matrix.rotate_right(new_start.row);
	Ok(())
}

fn parse_file(input_file: &str, matrix: &mut Vec<Vec<bool>>) -> Result<(), Error> {
    let input = File::open(input_file)?;
    let buffered = BufReader::new(input);
    let mut col;
    let mut row = 0;
	let mut start_point = Point {
		row: matrix.capacity(),
		col: matrix.capacity(),
	};
	let mut end_point = Point {
		row: 0,
		col: 0,
	};
    for line in buffered.lines() {//.expect("Cannot read file") {
        match line {
            Ok(l) => {
                col = 0;
                for c in l.chars() {
                    if row < matrix.capacity() &&
                       col < matrix[row].capacity() {
                        if c == '1' {
                            matrix[row][col] = true;
							if row < start_point.row {
								start_point.row = row;
							}
							if col < start_point.col {
								start_point.col = col;
							}
							if row > end_point.row {
								end_point.row = row;
							}
							if col > end_point.col {
								end_point.col = col;
							}
                        }
                        else {
                            matrix[row][col] = false;
                        }
                    }
                    else {
                        println!("Not handling this as yet\n");
                    }
                    col = col + 1;
                }
            }
            Err(e) => {
                println!("finished reading file! ({:?})", e)
            }
        }
        row = row + 1;
    }
    if let Err(_e) = center_matrix(matrix, start_point, end_point) {
		println!("Problem centering the matrix");
	}
    Ok(())
}

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

fn count_alive_neighbours(matrix: &Vec<Vec<bool>>, row_idx: usize, col_idx: usize) -> u32 {
	let start_row = if row_idx > 0 {row_idx - 1} else {(row_idx as isize - 1 + matrix.len() as isize) as usize};
	let start_col = if col_idx > 0 {col_idx - 1} else {(col_idx as isize - 1 + matrix[row_idx].len() as isize) as usize};
	let end_row = (row_idx + 1) % matrix.len();
	let end_col = (col_idx + 1) % matrix[row_idx].len();

	matrix[start_row][start_col] as u32 +
    matrix[start_row][col_idx] as u32 +
    matrix[start_row][end_col] as u32 +

	matrix[row_idx][start_col] as u32 +
    matrix[row_idx][end_col] as u32 +

	matrix[end_row][start_col] as u32 +
    matrix[end_row][col_idx] as u32 +
    matrix[end_row][end_col] as u32
}

fn update_matrix(matrix: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    /* Rules for the update:
     * - Any live cell with fewer than two live neighbours dies, as if by underpopulation.
     * - Any live cell with two or three live neighbours lives on to the next generation.
     * - Any live cell with more than three live neighbours dies, as if by
     * overpopulation.
     * - Any dead cell with exactly three live neighbours becomes a live cell, as if
     * by reproduction.
     *
     * Each cell has 8 neighbours and the plane has no boundaries (IE: wrap around)
     *
	 */
    let mut new_matrix = init_arena(matrix.len());
	for (row_idx, line) in matrix.iter().enumerate() {
		for (col_idx, cell) in line.iter().enumerate() {
			let num_alive_neighbours = count_alive_neighbours(&matrix, row_idx, col_idx);
			new_matrix[row_idx][col_idx] = *cell;
			if *cell {
				if num_alive_neighbours < 2 || num_alive_neighbours > 3 {
					new_matrix[row_idx][col_idx] = false;
				}
			} else {
				if num_alive_neighbours == 3 {
					new_matrix[row_idx][col_idx] = true;
				}
			}
		}
	}
	new_matrix
}

fn init_arena(asize: usize) -> Vec<Vec<bool>> {
    let mut matrix: Vec<Vec<bool>> = Vec::with_capacity(asize);
    for ii in 0..asize {
        matrix.push(Vec::with_capacity(asize));
        for _jj in 0..asize {
            matrix[ii].push(false)
        }
    }
	matrix
}

fn run(matches: ArgMatches) -> Result<(), String> {

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
    let mut matrix = init_arena(arena_size);

    if let Err(e) = parse_file(input_file, &mut matrix) {
        println!("Couldn't parse file, error {}", e);
    }
	let spin_sleeper = spin_sleep::SpinSleeper::new(100_000);
    for ii in 0..num_ticks {
        let new_matrix = update_matrix(&matrix);
    	update_screen(&matrix, ii);
		matrix = new_matrix;
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
