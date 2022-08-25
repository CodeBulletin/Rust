pub mod helper {
    pub fn print<T: std::fmt::Debug>(a: T) {
        println!("{:?}", a);
    }
}