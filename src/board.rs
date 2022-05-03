use std::fmt;

#[derive(Copy,Clone,PartialEq)]
pub enum Cell {
    Empty,
    Player1,
    Player2
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Cell::Empty => write!(f, "-"),
           Cell::Player1 => write!(f, "X"),
           Cell::Player2 => write!(f, "O"),
       }
    }
}

pub enum CellCheck {
    FoundSelf,
    FoundOther,
    FoundWinner
}

pub struct Board {
    board: Vec<Cell>,  // 2D board represented by 1D array
    game_ongoing: bool,
    winner: Cell
}

impl Board {
    const WIDTH: usize = 6;
    const HEIGHT: usize = 7;

    pub fn new() -> Board {
        Board {
            board: vec![Cell::Empty; Board::WIDTH * Board::HEIGHT],
            game_ongoing: true,
            winner: Cell::Empty
        }
    }

    pub fn print(&self) {
        for i in 0..Board::WIDTH {
            print!("{} ", i);
        }
        println!("");

        for y in 0..Board::HEIGHT {
            for x in 0..Board::WIDTH {
                print!("{} ", self.board[x + y * Board::WIDTH]);
            }
            println!("");
        }
        println!("");
    }

    pub fn game_ongoing(&self) -> bool {
        return self.game_ongoing;
    }

    pub fn get_winner(&self) -> Cell {
        return self.winner;
    }

    pub fn insert(&mut self, cell: Cell, x: usize) -> bool {
        if cell == Cell::Empty {
            eprintln!("Cannot insert empty cell!");
            return false;
        }

        if x >= Board::WIDTH {
            eprintln!("Cannot insert into row index {}. Max index is {}!", x, Board::WIDTH - 1);
            return false;
        }

        let mut insert_y = usize::MAX;
        for y in (0..Board::HEIGHT).rev() {
            if self.board[x + y * Board::WIDTH] == Cell::Empty {
                self.board[x + y * Board::WIDTH] = cell;
                insert_y = y;
                break;
            }
        }

        if insert_y == usize::MAX {
            eprintln!("Cannot insert cell to column {}. Column is full!", x);
            return false;
        }

        self.check_if_winning_insert(x, insert_y, &cell);
        return true;
    }

    fn check_cell(&mut self, x: usize, y: usize, cell: &Cell, num_in_a_row: &mut usize) -> CellCheck {
        let board_index: usize = x + y * Board::WIDTH;
        if self.board[board_index] != *cell {
            return CellCheck::FoundOther;
        }

        *num_in_a_row += 1;
        if *num_in_a_row < 4 {
            return CellCheck::FoundSelf;
        }

        self.winner = *cell;
        self.game_ongoing = false;
        return CellCheck::FoundWinner;
    }

    fn check_if_winning_insert(&mut self, x: usize, y: usize, cell: &Cell) {
        // Horizontal check
        let mut num_in_a_row: usize = 0;
        let mut x_check: usize = x;
        loop {
            match self.check_cell(x_check, y, cell, &mut num_in_a_row) {
                CellCheck::FoundSelf => if x_check == 0 { break } else { x_check -= 1 }
                CellCheck::FoundWinner => return,
                CellCheck::FoundOther => break
            }
        }
        x_check = x + 1;
        while x_check < Board::WIDTH {
            match self.check_cell(x_check, y, cell, &mut num_in_a_row) {
                CellCheck::FoundSelf => x_check += 1,
                CellCheck::FoundWinner => return,
                CellCheck::FoundOther => break
            }
        }

        // Vertical check
        num_in_a_row = 0;
        let mut y_check: usize = y;
        loop {
            match self.check_cell(x, y_check, cell, &mut num_in_a_row) {
                CellCheck::FoundSelf => if y_check == 0 { break } else { y_check -= 1 }
                CellCheck::FoundWinner => return,
                CellCheck::FoundOther => break
            }
        }
        y_check = y + 1;
        while y_check < Board::HEIGHT {
            match self.check_cell(x, y_check, cell, &mut num_in_a_row) {
                CellCheck::FoundSelf => y_check += 1,
                CellCheck::FoundWinner => return,
                CellCheck::FoundOther => break
            }
        }

        // / check
        num_in_a_row = 0;
        x_check = x;
        y_check = y;
        while y_check < Board::HEIGHT {
            match self.check_cell(x_check, y_check, cell, &mut num_in_a_row) {
                CellCheck::FoundSelf => if x_check == 0 { break } else { x_check -=1; y_check += 1; }
                CellCheck::FoundWinner => return,
                CellCheck::FoundOther => break
            }
        }
        num_in_a_row -= 1;
        x_check = x;
        y_check = y;
        while x_check < Board::WIDTH {
            match self.check_cell(x_check, y_check, cell, &mut num_in_a_row) {
                CellCheck::FoundSelf => if y_check == 0 { break } else { x_check += 1; y_check -= 1; }
                CellCheck::FoundWinner => return,
                CellCheck::FoundOther => break
            }
        }

        // \ check
        num_in_a_row = 0;
        x_check = x;
        y_check = y;
        loop {
            match self.check_cell(x_check, y_check, cell, &mut num_in_a_row) {
                CellCheck::FoundSelf => if x_check == 0 || y_check == 0 { break } else { x_check -= 1; y_check -= 1; }
                CellCheck::FoundWinner => return,
                CellCheck::FoundOther => break
            }
        }
        x_check = x + 1;
        y_check = y + 1;
        while x_check < Board::WIDTH && y_check < Board::HEIGHT {
            match self.check_cell(x_check, y_check, cell, &mut num_in_a_row) {
                CellCheck::FoundSelf => { x_check += 1; y_check += 1; }
                CellCheck::FoundWinner => return,
                CellCheck::FoundOther => break
            }
        }
    }
}