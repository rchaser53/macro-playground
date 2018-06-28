#[macro_use]
mod macro_test;

fn main() {
    println!("{}", sum!(1, 2, 33, 54));
    println!("{}", count!(1, 5, 2, 33, 54));
    println!("{}", average!(1, 2, 33, 48));
}


