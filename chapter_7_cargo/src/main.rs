use crate::garden::vegetables::Asparagus;

/* include code found at src/garden.rs */
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I am growing {:?}!", plant);
}
