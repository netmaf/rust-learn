fn main() {
    let v = vec![1, 2, 3];
    let iter= v.iter();

    for i in iter {
        println!("{}", *i);
    }

    println!("{:?}", v);
}
