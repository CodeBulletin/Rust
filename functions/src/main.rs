fn x(y: i128) {
    println!("{}", y);
}

fn fact(x: i128) -> i128 {
    if x == 0_i128 {
        return 1_i128;
    }
    else {
        return fact(x-1) * x;
    }
}

fn main() {
    let lambda = |i: i128| {
        return i / 2_i128;
    };
    x(lambda(fact(24)));
}
