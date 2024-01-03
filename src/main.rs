use crate::state::State;

mod state;

fn main() {
    let states = State {
        states: vec![1,2,3,4,5,6,7,8,9],
        active: 1,
    };
    for state in &states {
        println!("{}", state);
    }
    println!("{}",states.active());
}
