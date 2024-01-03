use crate::state::State;

mod state;

fn main() {
    let cell = State::new();

    println!("{}\n",cell.active());

    for element in cell.states().unwrap_or([0;9]).iter() {
        println!("{}",element);
    }

    println!("\nNumber of States: {}",cell.possible());
}
