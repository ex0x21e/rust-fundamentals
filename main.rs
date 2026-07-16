// The Read trait allows for reading bytes from a source.
// Implementors of the Read trait are called ‘readers’.
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
    .read_to_string(& mut input) //reads all bytes and converts to string
    .unwrap();

    let mut it = input.split_whitespace();

    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    println!("{}", a+b);
}
