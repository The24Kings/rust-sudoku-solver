use crate::state::State;
use crate::board::Board;
use crate::stack::Stack;

mod state;
mod board;
mod stack;

fn main() {
    let mut game = Board::new();

    // Mock setup for testing
    game.confirm(1, 8);
    game.confirm(7, 3);
    game.confirm(9, 2);
    game.confirm(12, 6);
    game.confirm(14, 7);
    game.confirm(17, 1);
    game.confirm(28, 6);
    game.confirm(30, 2);
    game.confirm(32, 1);
    game.confirm(34, 7);
    game.confirm(36, 5);
    game.confirm(44, 3);
    game.confirm(45, 9);
    game.confirm(48, 7);
    game.confirm(50, 5);
    game.confirm(53, 8);
    game.confirm(54, 4);
    game.confirm(56, 1);
    game.confirm(57, 3);
    game.confirm(59, 9);
    game.confirm(60, 7);
    game.confirm(62, 6);
    game.confirm(64, 2);
    game.confirm(70, 1);
    game.confirm(72, 8);
    game.confirm(74, 3);
    game.confirm(75, 1);
    game.confirm(77, 6);
    game.confirm(78, 5);
    game.confirm(80, 9);

    // Testing
/*
    let mut board = Board::new();
    let mut cell = State::new();
    let mut guesses: Stack<usize> = Stack::new(); // Stores the index of each State that we have already confirmd and guessed

    println!("{}\n",cell.active());

    for element in cell.states().iter() {
        println!("{}",element);
    }

    println!("\nStates: {:?}",cell.states());
    println!("Number of States: {}",cell.len());
    
    println!("\nRemoved: {}",cell.remove(8));
    println!("Popped: {}",cell.pop().unwrap());
    
    println!("\nStates: {:?}",cell.states());
    println!("Number of States: {}",cell.len());

    
    /*
    for state in board.board.iter() {
        println!("{}",state.active());
    }
    */

    board.cell(1).confirm(3);
    board.activate(2,4);

    println!("\n{}",board.cell(1).active());
    println!("{}",board.cell(2).active());

    guesses.push(3);
    guesses.push(2);
    guesses.push(5);
    guesses.push(7);
    guesses.push(9);

    println!("\n{:?}",guesses);

    println!("\nPopped: {}",guesses.pop().unwrap());
    println!("Last: {}",guesses.peek().unwrap());
    
    println!("\n{:?}",guesses);
*/
}
