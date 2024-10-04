
fn main() {
    println!("Hello, world 1!");
    println!("{}", fib(6));
}
// 0, 1, 2, 3, 4, 5, 6
// 0, 1, 1, 2, 3, 5, 8
fn fib(num: u32) -> u32 {

    if num == 0 {
        return 0;
    }

    if num == 1 {
        return 1;
    }

    let mut first: u32 = 0;
    let mut second: u32 = 1;

    for i in 1..num {
        let temp = second;
        second = second + first;
        first = temp;
    }

    return second;

}
