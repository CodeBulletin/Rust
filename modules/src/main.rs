mod functions;
mod vec;
use functions::helper::print;
use vec::Vec3;

fn main() {
    let v1 = Vec3(1.0_f32, 1.0, 1.0);
    let v2 = Vec3(2.0, 2.0, 3.0);

    println!("address of v1: {:p}", &v1);
    println!("address of v2: {:p}", &v2);

    let v3 = &v1 + &v2; //Borrows
    let v4 = v1 + v2; //Copies

    print(v1);
    print(v2);
    print(v3);
    print(v4);
}
