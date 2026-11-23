#[derive(Debug)]
enum Cell {
    X,
    O,
    Empty,
}

fn main() {
    let mut grid: [[Cell; 3]; 3] = [[Empty; 3]; 3]; //2d array
    print_current_grid(grid);
}

// fn checkWin: takes in a 3x3 grid, returns "X" or "O" for win, or false if no winner or tie.
// tie case is: all squares filled but checkWin returns false.

// fn findWinningRow
/*
fn find_winning_row(grid) -> Cell {
    'rows: for row in &grid {
        for cell in row {
            if cell != Cell::Empty && cell != &row[0] {
                continue 'rows;
            }
        }
        
        return *prev;
    }
    Cell::Empty
}
*/
// fn find_winning_column
// fn find_winning_diagonal


// fn takes coords as input to select where to go
//fn makeMove(grid: , x: u8, y: u8) -> () {
//    // TODO: make grid
//}


//fn print current grid
fn print_current_grid(grid: [[Cell; 3]; 3], coords: bool) -> () {
    let mut i = 0;
    for row in &grid {
        for &val in row{
            if coords{
                print!("{}: {}, {}", val, i/3 +1, i % 3 + 1);
                i+=1;
            }
            else{
                print!("{} ", val);
            }
        }
        println!();
    }
}