struct Cell {
    value: i32,
    row: i32,
    column: i32,
}

impl Cell {
    fn new(value: i32, row: i32, column: i32) -> Self {
        if value < 1 || value > 9 {
            panic!("Cell value must be between 1 and 9, inclusive.");
        }
        if row < 1 || row > 9 {
            panic!("Row value must be between 1 and 9, inclusive.");
        }
        if column < 1 || column > 9 {
            panic!("Column value must be between 1 and 9, inclusive.");
        }

        Cell {
            value,
            row,
            column,
        }
    }

    // https://stackoverflow.com/questions/5269064/sudoku-find-current-square-based-on-row-column
    fn get_ninth_part(&self) -> i32 {
        const WIDTH: i32 = 3;
        const NUM_MAJOR_COLUMNS: i32 = 3;

        let major_row = (&self.row - 1) / WIDTH;
        let major_column = (&self.column - 1) / WIDTH;

        major_column + major_row * NUM_MAJOR_COLUMNS + 1
    }
}

fn main() {
    let cell = Cell::new(1, 1, 1);
    let cell = Cell::new(0, 1, 1);
    println!("Hello, world!");
}
