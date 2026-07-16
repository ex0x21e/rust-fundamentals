// The Read trait allows for reading bytes from a source.
// Implementors of the Read trait are called ‘readers’.
use std::io::Read;

fn main() {
    //Rectangle Area
    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let width: i64 = it.next().unwrap().parse().unwrap();
    let height: i64 = it.next().unwrap().parse().unwrap();

    println!("{:.2}", width * height);
}
