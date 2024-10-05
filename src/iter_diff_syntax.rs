fn main() {
    let mut vec = vec![1, 2, 3];
    let mut iter = vec.iter_mut();

    while let Some(val) = iter.next() {
        println!("{}", val);
        *val = *val * 2;
    }

    println!("{:?}", vec);
}
