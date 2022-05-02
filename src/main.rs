mod board;
use std::io;

fn get_user_input() -> Option<usize> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            eprintln!("Invalid input! Please input a single number.");
            None
        }
    };
}

fn update_curr_player(cell: &mut board::Cell) {
    *cell = match cell {
        board::Cell::Player1 => board::Cell::Player2,
        board::Cell::Player2 => board::Cell::Player1,
        _ => {
            eprintln!("Current player has invalid value '{}'! Resetting to player 1's turn", cell);
            board::Cell::Player1
        }
    }
}

fn main() {
    let mut board = board::Board::new();
    let mut curr_player = board::Cell::Player1;
    board.print();
    while board.game_ongoing() {

        let input = get_user_input();
        if input == None {
            continue;
        }

        if board.insert(curr_player, input.unwrap()) {
            update_curr_player(&mut curr_player);
        }

        board.print();
    }

    match board.get_winner() {
        board::Cell::Empty => println!("It's a draw!"),
        board::Cell::Player1 => println!("Player 1 wins!"),
        board::Cell::Player2 => println!("Player 2 wins!")
    }
}
