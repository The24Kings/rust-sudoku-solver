use crate::state::State;
use crate::board::Board;
use crate::stack::Stack;

mod state;
mod board;
mod stack;

fn main() {
    let mut board = Board::new();
    let mut cell = State::new();
    let mut guesses: Stack<usize> = Stack::new(); // Stores the index of each State that we have already activated and guessed

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

    board.cell(1).activate(3);
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

}
