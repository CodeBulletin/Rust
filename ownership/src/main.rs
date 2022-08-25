fn main() {
    // ----- Ownership rules -----
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There Can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    { // s is not valid
        let s: &str = "hello";
        println!("{}", s)
        //s is valid here
    } // scope is over and s is no longer valid cant do println!("{}", s);

    //Primitive Copy
    let x: i32 = 5;
    let y = x; //Copy the value

    println!("x: {}, y: {}", x, y);

    let s1 = String::from("hello ownership");
    let s2 = s1; //Rust invalidates the s1 variable and gives the ownership to s2;

    // println!("{}", s1); wont work because s1 is no longer valid
    println!("{}", s2);

    let s1 = String::from("hello copy");
    let s2 = s1.clone(); //Cloning the value to tranfering ownership

    println!("{}", s1);
    println!("{}", s2);

    let s = String::from("hello ownership");
    takes_ownership(s);
    // println!("{}", s); wont work because s is no longer valid

    let x = 0;
    makes_copy(x);
    println!("{}", x);

    let mut s = String::from("hello ownership");
    s = takes_ownership_and_back(s);
    println!("{}", s);

}

fn takes_ownership(str: String) { // takes the ownership of the variable
    println!("{}", str);
} // delete the variable

fn makes_copy(x: i32) { // takes the ownership of the variable
    println!("{}", x);
} // delete the variable

fn takes_ownership_and_back(str: String) -> String {
    println!("{} i am going to return it back", str);
    return str;
}