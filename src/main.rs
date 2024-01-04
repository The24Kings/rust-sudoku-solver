use crate::state::State;
use crate::board::Board;

mod state;
mod board;

fn main() {
    let board = Board::new();
    let cell = State::new();

    println!("{}\n",cell.active());

    for element in cell.states().iter() {
        println!("{}",element);
    }

    println!("\nNumber of States: {}",cell.len());

    for state in board.board.iter() {
        println!("{}",state.active());
    }

}
