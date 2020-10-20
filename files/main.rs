// use std::ops::Add;

// impl Add<u64> for u32 {
//     type Output = u64;
//     fn add(self) -> Output {
//         self as u64 + other
//     }
// }

// fn main() {
//     let a1 = 1u32;
//     let b = 2u64;
//     assert_eq!(a1 + b, 3);
// }

// trait Add<RHS, Output> {
//     fn my_add(self, rhs: RHS) -> Output;
// }

// impl Add<i32, i32> for i32 {
//     fn my_add(self, rhs: i32) -> i32 {
//         self + rhs
//     }
// }

// impl Add<u32, i32> for u32 {
//     fn my_add(self, rhs: u32) -> i32 {
//         (self + rhs) as i32
//     }
// }

pub trait Add<RHS = Self> {
    type Output;
    fn my_add(self, rhs: RHS) -> Self::Output;
}

impl Add for i32 {
    type Output = i32;
    fn my_add(self, other) -> i32 {
        self + other
    }
}

fn main() {
    let (a, b) = (1i32, 2i32);
    let x: i32 = a.my_add(b);
    assert_eq!(x, 3i32);
    // let y: i32 = c.my_add(d);
    // assert_eq!(y, 7i32);
    println!("{:?}", a);
}
