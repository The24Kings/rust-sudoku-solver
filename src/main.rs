use crate::state::State;

mod state;

fn main() {
    let cell = State::new();

    println!("{}\n",cell.active());

    for element in cell.states().iter() {
        println!("{}",element);
    }

    println!("\nNumber of States: {}",cell.len());
}
