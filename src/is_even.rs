// create function is_even that takes a number as an input and returns true if its even


fn main() {
    println!("{}", is_even(10));
    println!("Hello, world2 3!");
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return  false;
}
