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
use std::ops::Shr;
use std::ops::Sub;
use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};
use spin_sleep;
const ARENA_SIZE: usize = 40;

struct Point {
	row: usize,
	col: usize,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, p: Point) -> Point {
        let r = Point {
            row:  (self.row as i64 - p.row as i64).abs() as usize,
            col:  (self.col as i64 - p.col as i64).abs() as usize
        };
        return r
    }
}
impl Sub<usize> for Point {
    type Output = Self;

    fn sub(self, s: usize) -> Point {
        let r = Point {
            row:  (self.row as i64 - s as i64).abs() as usize,
            col:  (self.col as i64 - s as i64).abs() as usize
        };
        return r
    }
}

impl Shr<usize> for Point {
    type Output = Self;

    fn shr(self, shr: usize) -> Self {
        Self {row: self.row >> shr, col: self.col >> shr}
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Matrix {
    m: Vec<Vec<bool>>,
    size: usize
}

impl Matrix {

    pub fn new(size: usize) -> Self {
        Self { m: Vec::with_capacity(size), size: size }
    }

    fn capacity(&self) -> usize {
        self.size
    }

    fn init(&mut self) {
        for ii in 0..self.size {
            self.m.push(Vec::with_capacity(self.size));
            for _jj in 0..self.size {
                self.m[ii].push(false)
            }
        }
    }
    fn update_matrix(&self) -> Matrix {
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
        let mut new_m = self.clone();
        let m = self.m.clone();
    	for (row_idx, line) in m.iter().enumerate() {
    		for (col_idx, cell) in line.iter().enumerate() {
    			let num_alive_neighbours = self.count_alive_neighbours(row_idx, col_idx);
    			new_m.m[row_idx][col_idx] = *cell;
    			if *cell {
    				if num_alive_neighbours < 2 || num_alive_neighbours > 3 {
    					new_m.m[row_idx][col_idx] = false;
    				}
    			} else {
    				if num_alive_neighbours == 3 {
    					new_m.m[row_idx][col_idx] = true;
    				}
    			}
    		}
    	}
        return new_m;
    }
    fn count_alive_neighbours(&self, row_idx: usize, col_idx: usize) -> u32 {
        assert!(row_idx < self.m.len());
        assert!(col_idx < self.m[row_idx].len());
    	let start_row = if row_idx > 0 {row_idx - 1} else {(row_idx as isize - 1 + self.m.len() as isize) as usize};
    	let start_col = if col_idx > 0 {col_idx - 1} else {(col_idx as isize - 1 + self.m[row_idx].len() as isize) as usize};
    	let end_row = (row_idx + 1) % self.m.len();
    	let end_col = (col_idx + 1) % self.m[row_idx].len();

    	self.m[start_row][start_col] as u32 +
        self.m[start_row][col_idx] as u32 +
        self.m[start_row][end_col] as u32 +

    	self.m[row_idx][start_col] as u32 +
        self.m[row_idx][end_col] as u32 +

    	self.m[end_row][start_col] as u32 +
        self.m[end_row][col_idx] as u32 +
        self.m[end_row][end_col] as u32
    }
}

fn center(matrix: &mut Matrix , start: Point, end: Point) {
    let half_length = (end - start) >> 1;
    let new_start = half_length - matrix.capacity();
    let m = &mut matrix.m;
	for line in &mut m.into_iter() {
		line.rotate_right(new_start.col);
	}
	matrix.m.rotate_right(new_start.row);
}

fn parse_file(input_file: &str, arena_size: usize) -> Result<Matrix, Error> {
    let mut matrix = Matrix::new(arena_size);
    matrix.init();
    let input = File::open(input_file)?;
    let buffered = BufReader::new(input);
    let mut col;
    let mut row = 0;
	let mut start_point = Point {
		row: arena_size,
		col: arena_size,
	};
	let mut end_point = Point {
		row: 0,
		col: 0,
	};
    for line in buffered.lines() {
        match line {
            Ok(l) => {
                col = 0;
                for c in l.chars() {
                    if row < matrix.capacity() &&
                       col < matrix.m[row].capacity() {
                        if c == '1' {
                            matrix.m[row][col] = true;
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
                            matrix.m[row][col] = false;
                        }
                    }
                    else {
                        println!("arena size too small for file! row {}, col {}, capacity {}, row cap {}",
                                 row, col, matrix.capacity(), matrix.m[row].capacity());
                        return Err(Error::new(ErrorKind::InvalidInput, "arena too small"));
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
    center(&mut matrix, start_point, end_point);
    Ok(matrix)
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

    let mut matrix = parse_file(input_file, arena_size)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_init() {
        let arena_size = 5;
        let matrix = Matrix::new(arena_size);
        assert_eq!(matrix.capacity(), 5);
    }

    #[test]
    fn check_parse_file_valid() {
       let input_file = "./data/toad.txt";
       let matrix = parse_file(input_file, 7).unwrap();
       let data_correct = vec![
           [false, false, false, false, false, false, false],
           [false, true,  true,  true,  false, false, false],
           [true,  true,  true,  false, false, false, false],
           [false, false, false, false, false, false, false],
           [false, false, false, false, false, false, false],
           [false, false, false, false, false, false, false],
           [false, false, false, false, false, false, false],
       ];
       assert_eq!(data_correct.len(), matrix.m.len());
       for lines in data_correct.iter().zip(matrix.m.iter())
       {
           let (correct_line, ut_line) = lines;
           for cols in correct_line.iter().zip(ut_line.iter())
           {
               let (correct_col, ut_col) = cols;
               assert_eq!(correct_col, ut_col);
           }
       }
    }

    #[test]
    fn check_parse_file_matrix_too_small() {
       let input_file = "./data/toad.txt";
       let matrix = parse_file(input_file, 5).map_err(|e|e.kind());
       assert_eq!(matrix, Err(ErrorKind::InvalidInput));
    }

    #[test]
    fn check_alive_neighbours_good() {
       let input_file = "./data/toad.txt";
       let matrix = parse_file(input_file, 7).unwrap();
       // Beginning
       assert_eq!(matrix.count_alive_neighbours(0, 0), 1);
       // End
       assert_eq!(matrix.count_alive_neighbours(4, 4), 0);
       // somewhere in the centre
       assert_eq!(matrix.count_alive_neighbours(2, 2), 4);
       // different indexes
       assert_eq!(matrix.count_alive_neighbours(2, 4), 1);
    }

    #[test]
    #[should_panic]
    fn check_alive_neighbours_bad() {
       let input_file = "./data/toad.txt";
       let matrix = parse_file(input_file, 7).unwrap();
       // too big an index
       assert_eq!(matrix.count_alive_neighbours(7, 9), 1);
    }

    #[test]
    fn check_update_matrix() {
       let input_file = "./data/toad.txt";
       let matrix = parse_file(input_file, 7).unwrap();
       let new_m = matrix.update_matrix();
       let data_correct = vec![
           [false, false, true,  false, false, false, false],
           [true,  false, false, true,  false, false, false],
           [true,  false, false, true,  false, false, false],
           [false, true,  false, false, false, false, false],
           [false, false, false, false, false, false, false],
           [false, false, false, false, false, false, false],
           [false, false, false, false, false, false, false],
       ];
       assert_eq!(data_correct.len(), new_m.m.len());
       for lines in data_correct.iter().zip(new_m.m.iter())
       {
           let (correct_line, ut_line) = lines;
           for cols in correct_line.iter().zip(ut_line.iter())
           {
               let (correct_col, ut_col) = cols;
               assert_eq!(correct_col, ut_col);
           }
       }
    }

}
