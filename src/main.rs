use crate::state::State;
use crate::board::Board;

mod state;
mod board;

fn main() {
    let mut board = Board::new();
    let cell = State::new();

    println!("{}\n",cell.active());

    for element in cell.states().iter() {
        println!("{}",element);
    }

    println!("\nNumber of States: {}",cell.len());
    
    /*
    for state in board.board.iter() {
        println!("{}",state.active());
    }
    */

    board.cell(1).activate(3);
    board.activate(2,4);

    println!("{}",board.cell(1).active());
    println!("{}",board.cell(2).active());

}
