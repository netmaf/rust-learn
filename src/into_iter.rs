// into iter takes ownership of value

fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.into_iter();

    while let Some(val) = iter.next() {
        println!("{}", val)
    }

    
    // normal for loop is into_iter()
    // println!("{:?}", v); //cant print value

}
