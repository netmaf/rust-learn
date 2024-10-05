use std::{slice::from_raw_parts, str::from_utf8};

fn main() {
    let mut name = String::from("Hello");
    name.push('a');
    println!("{}", name);

    let addr = name.as_ptr();
    println!("{:p}", addr);
    unsafe {
        let len = name.len();

        // create a slice

        let slice = from_raw_parts(addr, len);
        println!("{:?}", slice);
        if let Ok(str) = from_utf8(slice) {
            println!("Def str -> {}", str)
        } else {
            println!("add mis")
        }
    }
}
