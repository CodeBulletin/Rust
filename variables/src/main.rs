fn main() {
    let mut x = 4;
    println!("x is: {}", x);
    {
        x = 5;
    }
    println!("x is: {}", x);


    // integer types:
        // unsigned: u8, u16, u32, u64, u128
        // signed: i8, i16, i32, i64, i128
    
    // floating point:
        // f32, f64
    
    // Character:
        // char -> 'a'
    
    // Boolean
        // bool -> false/ true
    
    // Tuples
        // let tupe: (i32, i32, bool) = (0, 1, false)
    
    //Arrays
        // let array: [i32; 5] = [1, 2, 3, 4, 5]
        
    let mut array: [i32; 5] = Default::default();
    array[1] += 1;
    println!("{}, {}", array[1], array[2]);
}
