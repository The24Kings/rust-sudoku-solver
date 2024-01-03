use crate::state::State;

mod state;

fn main() {
    let cell = State::new();

    println!("{}",cell.active());

    for element in cell.states().into_iter() {
        println!("{:?}",element);
    }

    println!("Number of States: {}",cell.states()
        .expect("No States Available!")
        .len()
    );
}
