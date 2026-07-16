fn main() {
    // Multiples of 3 → Fizz,
    // of 5 → Buzz,
    // both → FizzBuzz,
    // else the number.
    // The classic ordering trap applies — 
    // check the "both" case first (first true branch wins; test 3 before 15 and the 15 case is unreachable).
    // Two idiomatic shapes; pick either:

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n:i32 = buf.trim().parse().unwrap();

    match (n % 3 == 0, n % 5 == 0){
        (true, true) => println!("FizzBuzz"),
        (true, false) => println!("Fizz"),
        (false, true) => println!("Buzz"),
        _ => println!("{}", n)
    };
}
