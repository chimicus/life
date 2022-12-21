use crate::point::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub m: Vec<Vec<bool>>,
    size: usize
}

impl Matrix {

    pub fn new(size: usize) -> Self {
        Self { m: Vec::with_capacity(size), size: size }
    }

    pub fn capacity(&self) -> usize {
        self.size
    }

    pub fn init(&mut self) {
        for ii in 0..self.size {
            self.m.push(Vec::with_capacity(self.size));
            for _jj in 0..self.size {
                self.m[ii].push(false)
            }
        }
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
