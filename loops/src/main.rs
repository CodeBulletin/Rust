use std::io::Write;

fn main() {
    let mut input = String::new();

    print!("Enter the number: ");

    std::io::stdout().flush().expect("cannot flush error!");

    std::io::stdin().read_line(&mut input).expect("cannot read input error!");

    let x: i32 = input.trim().parse().unwrap();

    for i in (0..x).step_by(2) {
        for _ in (0..i+2).step_by(2) {
            print!("*");
        }
        println!("");
    }

    let mut i: i32 = 0;
    while i < x {
        let mut j: i32 = 0;
        while j < i+2 {
            print!("*");
            j+=2;
        }
        println!("");
        i+=2;
    }

    i = 0;
    loop {
        if i >= x {
            break;
        } 
        let mut j: i32 = 0;
        loop {
            if j >= i+2 {
                break;
            }
            print!("*");
            j+=2
        }
        println!("");
        i+=2;
    }
}
