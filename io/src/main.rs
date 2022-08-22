use std::io::Write;

use std::io;

fn main() {
    let mut input = String::new();
    print!("enter your integer: ");
    io::stdout().flush().expect("unable to flush");
    io::stdin().read_line(&mut input).expect("failed to read line");

    let int_input: i64 = input.trim().parse().unwrap();
    println!("integer + 1: {}", int_input + 1);
}
