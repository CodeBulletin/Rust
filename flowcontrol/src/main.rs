fn main() {
    let food = "pizza";
    if food == "cookie" {
        println!("cookie from if");
    }
    else if food != "pizza" {
        println!("healthy food from if");
    }
    else {
        println!("eww form if");
    }

    match food {
        "cookie" => println!("yay cookie"),
        _ => {
            println!("no cookie :(");
        }
    }
}
