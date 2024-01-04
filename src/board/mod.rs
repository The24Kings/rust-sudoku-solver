use crate::state::State;

#[derive(Debug, Clone)]
pub struct Board {
    pub board: Vec<State>,
}

impl Board {
    pub fn new() -> Self {
        let mut new_board = vec!();

        for _ in 0..81 {
            new_board.push(State::new());
        }

        Board {
            board: new_board,
        }
    }

    pub fn cell(&mut self, index: usize) -> &mut State {
        &mut self.board[index]
    }

    pub fn activate(&mut self, index: usize, num: u16) {
        self.board[index].active = num;
    }

}

