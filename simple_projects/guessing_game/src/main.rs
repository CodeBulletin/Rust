use std::fmt::Debug;
use std::str::FromStr;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn take_input<T: FromStr>(msg: &str) -> T where <T as FromStr>::Err: Debug {
    print!("{}", msg);
    let mut str = String::new();
    std::io::stdout().flush().expect("unable to flush");

    std::io::stdin().read_line(&mut str).expect("unable to take input");

    let val: T =  str.trim().parse().unwrap();
    return val;
}

fn main() {
    loop {
        let actual_value: i32 = rand::thread_rng().gen_range(1..=100);
        let mut done: bool = false;
        while !done {
            let guess: i32 = take_input("Enter the number: ");
            
            match guess.cmp(&actual_value) {
                Ordering::Less => println!("Try Again! you guessed lower then the actual value"),
                Ordering::Greater => println!("Try Again! you guessed lower then the actual value"),
                _ => {
                    println!("Yay you guessed correctly");
                    done = true;
                }
            }
        }
    }
}