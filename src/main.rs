use crate::state::State;

mod state;

fn main() {
    let cell = State::new();

    println!("{}",cell.active());

    for element in cell.states().into_iter() {
        println!("{:?}",element);
    }
}
