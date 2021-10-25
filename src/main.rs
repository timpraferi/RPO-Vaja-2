use rand::prelude::*;

fn main() {
    let rand: u8  = random::<u8>() % 6 + 1;
    println!("Met kocke je {}", rand);
}
