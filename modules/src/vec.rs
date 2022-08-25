
#[derive(Debug, Clone, Copy)]
pub struct Vec3<T: std::ops::Add>(pub T, pub T, pub T);

impl<T: std::ops::Add<Output = T> + Copy> std::ops::Add for &Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, _rhs: &Vec3<T>) -> Vec3<T> {
        println!("address of v1 from &v1 + &v2: {:p}", self);
        println!("address of v2 from &v1 + &v2: {:p}", _rhs);
        return Vec3(self.0 + _rhs.0, self.1 + _rhs.1, self.2 + _rhs.2);
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, _rhs: Vec3<T>) -> Vec3<T> {
        println!("address of v1 from v1 + v2: {:p}", &self);
        println!("address of v1 from v1 + v2: {:p}", &_rhs);
        return Vec3(self.0 + _rhs.0, self.1 + _rhs.1, self.2 + _rhs.2);
    }
}