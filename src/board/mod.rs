use crate::state::State;
use std::fmt;

pub enum Solution {
    Solved,
    UnSolvable,
}

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

    pub fn solve(&mut self) -> Solution { 
        loop {
            // Check cells with only 1 State
            for _i in 0..81 {
                let state = self.cell(_i);

                if state.is_known() { continue; }

                if state.len() != 1 { continue; }

                println!("Confirming {} at {}",state.active(),_i);

                self.confirm(_i, state.states()[0]);
            }

            break; // Testing
        }
        
        Solution::Solved
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
    pub fn cell(&self, index: usize) -> &State {
        &self.board[index]
    }

    pub fn confirm(&mut self, index: usize, num: u16) {
        self.board[index] = State {
            states: Vec::new(),
            active: num,
            known: true,
        };

        self.propagate_states(index, num);
    }

    pub fn activate(&mut self, index: usize, num: u16) {
        self.board[index].active = num;
    }
    
    pub fn check(&self, num: u16, index: usize) -> bool {
        self.check_row(num, index) && self.check_col(num, index) && self.check_box(num, index)
    }

    pub fn propagate_states(&mut self, index: usize, num: u16) {
        self.update_row(index, num);
        self.update_col(index, num);
        self.update_box(index, num);
    }

    pub fn is_known(&self, index: usize) -> bool {
        self.board[index].known
    }

    fn row(&self, index: usize) -> Vec<State> {
        let start = index / 9;
        let end = start + 9;

        self.board[start..end].to_vec()
    }

    fn col(&self, index: usize) -> Vec<&State> {
        let start = index / 9;

        let mut output: Vec<&State> = Vec::new();

        for _i in 0..9 {
            output.push(&self.board[start + (_i * 9)]);
        }

        output
    }

    fn group(&self, index: usize) -> Vec<&State> {
        let mut output: Vec<&State> = Vec::new();
        let x_offset = index % 3;
        let y_offset = (index / 9) % 3;

        let mut corner = index - x_offset - (9 * y_offset);

        for _i in 0..9 {
            output.push(&self.board[corner]);

            if _i % 3 == 2 && _i != 8 {
                corner += 7;
            } else {
                corner += 1;
            }
        }

        output
    }

    fn check_row(&self, num: u16, index: usize) -> bool {
        for element in self.row(index) {
            if num == element.active() {
                return false;
            }
        }
        
        true
    }

    fn check_col(&self, num: u16, index: usize) -> bool {
        for element in self.col(index) {
            if num == element.active() {
                return false;
            }
        }
        
        true 
    }

    fn check_box(&self, num: u16, index: usize) -> bool {
        for element in self.group(index) {
            if num == element.active() {
                return false;
            }
        } 
        
        true
    }

    fn update_row(&mut self, index: usize, num: u16) {
        let length = self.row(index).len();
        println!("\nUpdating Row {} with {}",index,num);

        for _j in 0..length {
            if self.row(index)[_j].is_known() {
                println!("\nKnown");
                continue;
            }
            
            let state_index = self.row(index)[_j].states().iter().position(|x| *x == num);

            if let Some(_i) = state_index {
                println!("\nBefore {:?}",self.row(index)[_j].states());
                self.row(index)[_j].states().remove(state_index.unwrap());

            } else {
                panic!("Specified State cannot be found!");

            }

            println!("State Index: {:?}",state_index);

            println!("After  {:?}",self.row(index)[_j].states());
        }
    }
    
    fn update_col(&mut self, index: usize, num: u16) {
        /*
        for element in self.col(index) {
            let state_index = element.states().iter().position(|x| *x == num);

            if let Some(_i) = state_index {
                element.remove(state_index.unwrap());
            } else if element.is_known() {
                continue;
            } else {
                panic!("Specified State cannot be found!");
            }
        }
        */
    }

    fn update_box(&mut self, index: usize, num: u16) {
        /*
        for mut element in self.group(index) {
            let state_index = element.states().iter().position(|x| *x == num);

            if let Some(_i) = state_index {
                element.remove(state_index.unwrap());
            } else if element.is_known() {
                continue;
            } else {
                panic!("Specified State cannot be found!");
            }
        }*/
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        for _i in 0..81 {
            let input = &self.board[_i].active().to_string();

            if _i % 27 == 0 && _i != 0 { output.push_str("- - - | - - - | - - -\n"); }

            if input == "0" {
                output.push_str("~ ");
            } else {
                output.push_str(input);
                output.push_str(" ");
            }

            if _i == 0 && _i / 9 == 0 { continue; }

            if _i % 9 == 8 { 
                output.push_str("\n"); 
                continue; 
            }

            if _i % 3 == 2 { output.push_str("| "); }
        }

        write!(f, "{}", output)
    }
}

