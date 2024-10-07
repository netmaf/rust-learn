use std::thread::spawn;

fn main() {
    let v = vec![1, 2, 3];

    // use move as v can go out of scope and thread might still try to access it
    spawn(move || {
        println!("{:?}", v)
    }).join().unwrap();
}
