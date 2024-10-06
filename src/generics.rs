

fn main() {
    let ans1 = get_largest(10, 20);
    let ans2 = get_largest('a', 'b');

    println!("{}, {}", ans1, ans2)
}

fn get_largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    }

    return b;
}
