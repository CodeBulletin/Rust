struct Range<T: std::cmp::PartialOrd + Copy + std::ops::AddAssign> {
    start: T,
    end: T,
    step: T,
    current: T,
    reverse: bool
}

impl<T: std::cmp::PartialOrd + Copy + std::ops::AddAssign> Range<T> {
    pub fn new(start: T, end: T, step: T) -> Range<T> {
        return Range {
            start: start,
            end: end,
            step: step,
            current: start,
            reverse: {start > end}
        };
    }
}

impl<T: std::cmp::PartialOrd + Copy + std::ops::AddAssign> Iterator for Range<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reverse {
            if self.current <= self.start && self.current > self.end {
                let val = Some(self.current);
                self.current += self.step;
                return val;
            }
            return None;
        }
        if self.current < self.end && self.current >= self.start {
            let val = Some(self.current);
            self.current += self.step;
            return val;
        }
        return None;
    }
}

fn main() {
    // println!("Hello, world!");
    let collection = Range::new(-1i128, 1000, 1).filter(|x| *x < 10i128);
    for i in collection {
        println!("{}", i)
    }
}
