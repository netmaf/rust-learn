fn main() {
    let mut v = vec![1, 2, 3];
    let iter= v.iter_mut();

    for i in iter {
        *i = *i * 2;
    }

    println!("{:?}", v);
}
