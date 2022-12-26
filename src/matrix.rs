use crate::point::Point;
use std::io::{BufReader, BufRead, Error, ErrorKind};
use std::fs::File;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub m: Vec<Vec<bool>>,
    size: usize
}

impl Matrix {

    pub fn new(size: usize) -> Matrix {
        assert!(size > 0);
        let mut m = Vec::with_capacity(size);
        for ii in 0..size {
            m.push(Vec::with_capacity(size));
            for _jj in 0..size {
                m[ii].push(false)
            }
        }
        Matrix { m, size: size }
    }

    pub fn capacity(&self) -> usize {
        self.size
    }

    pub fn parse_file(&mut self, input_file: &str) -> Result<(), Error> {
        let input = File::open(input_file)?;
        let buffered = BufReader::new(input);
        let mut col;
        let mut row = 0;
    	let mut start_point = Point {
    		row: self.capacity(),
    		col: self.capacity(),
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
                        if row < self.capacity() &&
                           col < self.m[row].capacity() {
                            if c == '1' {
                                self.m[row][col] = true;
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
                                self.m[row][col] = false;
                            }
                        }
                        else {
                            println!("arena size too small for file! row {}, col {}, capacity {}, row cap {}",
                                     row, col, self.capacity(), self.m[row].capacity());
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
        self.center(start_point, end_point);
        Ok(())
    }
    pub fn update_matrix(&self) -> Matrix {
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
    pub fn count_alive_neighbours(&self, row_idx: usize, col_idx: usize) -> u32 {
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

    pub fn center(&mut self, start: Point, end: Point) {
        let half_length = (end - start) >> 1;
        let new_start = half_length - self.capacity();
        let m = &mut self.m;
    	for line in &mut m.into_iter() {
    		line.rotate_right(new_start.col);
    	}
    	self.m.rotate_right(new_start.row);
    }
}

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
       let mut matrix = Matrix::new(7);
       matrix.parse_file(input_file).unwrap();
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
       let mut matrix = Matrix::new(5);
       let res = matrix.parse_file(input_file).map_err(|e|e.kind());
       assert_eq!(res, Err(ErrorKind::InvalidInput));
    }

    #[test]
    fn check_alive_neighbours_good() {
       let input_file = "./data/toad.txt";
       let mut matrix = Matrix::new(7);
       matrix.parse_file(input_file).unwrap();
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
       let mut matrix = Matrix::new(7);
       matrix.parse_file(input_file).unwrap();
       // too big an index
       assert_eq!(matrix.count_alive_neighbours(7, 9), 1);
    }

    #[test]
    fn check_update_matrix() {
       let input_file = "./data/toad.txt";
       let mut matrix = Matrix::new(7);
       matrix.parse_file(input_file).unwrap();
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

