use crate::state::State;

mod state;

#[derive(Clone)]
pub struct Board {
    pub board: Vec<State>,
}

impl Board {
    pub fn new() -> Board {
        let mut new_board = vec!();

        for _ in 0..81 {
            new_board.push(State::new());
        }

        Board {
            board: new_board,
        }
    }
}

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
