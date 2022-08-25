#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u128,
    active: bool 
}

//Tuple like structs

#[derive(Debug)]
struct Vec3(f32, f32, f32);

#[derive(Debug)]
struct Pair<T, F>(T, F);

#[derive(Debug)]
struct Color(u8, u8, u8, u8);

impl Color {
    fn hex(&self) -> u32 {
        return u32::from_be_bytes([self.0, self.1, self.2, self.3]);
    }

    fn from_hex(hex: u32) -> Color {
        let bytes = u32::to_be_bytes(hex);
        return Color(bytes[0], bytes[1], bytes[2], bytes[3]);
    }
}

fn main() {
    let user1 = User {
        email: String::from("XYZ"),
        username: String::from("WX"),
        sign_in_count: 22,
        active: false
    };
    // let s1 = user1.email; cannot do this if remeber ownership
    println!("{} {} {} {}", user1.email, user1.username, user1.sign_in_count, user1.active);

    let user2 = create_user(String::from("abc"), String::from("xyz"));

    println!("{:?}", user2);

    let user3 = User {
        email: String::from("pqr"),
        ..user2
    }; // we cant user user2 now
    println!("{:?}", user3);

    let black = Color(170, 46, 182, 250);
    println!("{:?}", black);

    let point = Vec3(1.2, 1.3, 1.4);
    println!("{:?}", point);

    let p: Pair<i32, f64> = Pair(1, 1.0);
    println!("{:?}", p);

    let hex = black.hex();
    println!("{:#010x}", hex);

    println!("{:?}", Color::from_hex(0xaa2eb6fa));
}

fn create_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true
    }
}
