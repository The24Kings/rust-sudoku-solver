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
/*
    pub fn coords(index: usize) -> [u16;2] {
        [(index%9),(index/9)] // x, y
    }

    pub fn x(%self, index: usize) - u16 {
        index % 9
    }
    
    pub fn y(%self, index: usize) - u16 {
        index / 9
    }
*/
    pub fn cell(&mut self, index: usize) -> &mut State {
        &mut self.board[index]
    }

    pub fn confirm(&mut self, index: usize, num: u16) {
        self.board[index] = State {
            states: vec!(),
            active: num,
            known: true,
        }
    }
/*    
    pub fn confirm(&mut self, index: usize,  num: u16) {
        self.board[index].states = vec!();
        self.board[index].active = num;
        self.board[index].known = true;
    }
*/

    pub fn activate(&mut self, index: usize, num: u16) {
        self.board[index].active = num;
    }

    pub fn row(&self, index: usize) -> Vec<State> {
        let start = index % 9;
        let end = start + 9;

        self.board[start..end].to_vec()
    }

    pub fn col(&self, index: usize) -> Vec<&State> {
        let start = index / 9;

        let mut output: Vec<&State> = vec!();

        for _i in 0..9 {
            output.push(&self.board[start + (_i * 9)]);
        }

        output
    }

    pub fn group(&self, index: usize) -> Vec<&State> {
        let mut output: Vec<&State> = vec!();
        let x_offset = index % 3;
        let y_offset = (index / 9) % 3;
        
        println!("X Offset: {}",x_offset);
        println!("Y Offset: {}",y_offset);

        // Panics
        let mut corner = index - x_offset - y_offset;

        println!("Corner @ {}",corner);

        for _i in 0..9 {
            if _i / 3 == 0 && _i != 0 {
                corner += 9;
            }

            output.push(&self.board[corner]);
            
            corner += 1;

            println!("Loop: {}",_i);
        }

        output
    }

    pub fn is_known(&self, index: usize) -> bool {
        self.board[index].known
    }

    pub fn check(&self, num: u16, index: usize) -> bool {
        self.check_row(num, index) && self.check_col(num, index) && self.check_box(num, index)
    }

    pub fn check_row(&self, num: u16, index: usize) -> bool {
        for element in self.row(index) {
            if num == element.active() {
                return false;
            }
        }
        
        println!("Row Success.");
        true
    }

    pub fn check_col(&self, num: u16, index: usize) -> bool {
        for element in self.col(index) {
            if num == element.active() {
                return false;
            }
        }
        
        println!("Col Success.");
        true 
    }

    pub fn check_box(&self, num: u16, index: usize) -> bool {
        for element in self.group(index) {
            if num == element.active() {
                return false;
            }
        } 
        
        println!("Box Success.");
        true
    }
}

