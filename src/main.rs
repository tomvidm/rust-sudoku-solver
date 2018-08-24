extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod su;

fn main() {
    let board = su::Board::new();
    println!("Hello, world!");
}
