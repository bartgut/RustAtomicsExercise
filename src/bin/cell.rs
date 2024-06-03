mod bin;

use std::cell::Cell;

struct Grid {
    cells: Vec<Vec<Cell<i32>>>
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            cells: vec![vec![Cell::new(0); cols];rows]
        }
    }

    fn set_value(&self, row: usize, col: usize, value: i32) -> &Self {
        let cell = self.cells.get(row).map(|row| row.get(col)).flatten().unwrap();
        cell.set(value);
        self
    }

    fn get_value(&self, row: usize, col: usize) -> i32 {
        let cell = self.cells.get(row).map(|row| row.get(col)).flatten().unwrap();
        cell.get()
    }

    fn print_grid(&self) {
        for row in &self.cells {
            for col in row {
                println!("{}", col.get())
            }
        }
    }

    fn sum_row(&self, row: usize) -> i32 {
        let cells = &self.cells[row];
        cells.iter().map(|cell| cell.get()).sum()
    }

}


fn main() {
    let grid = Grid::new(3, 3);

    // Set some values in the grid
    grid.set_value(0, 0, 1);
    grid.set_value(1, 1, 2);
    grid.set_value(2, 2, 3);

    // Get and print some values from the grid
    println!("Value at (0, 0): {}", grid.get_value(0, 0));
    println!("Value at (1, 1): {}", grid.get_value(1, 1));
    println!("Value at (2, 2): {}", grid.get_value(2, 2));

    println!("Row sum: {}", grid.sum_row(2));

    // Print the entire grid
    grid.print_grid();
}
