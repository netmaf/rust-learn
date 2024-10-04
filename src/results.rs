use std::fs::read_to_string;

fn main() {
    println!("Hello, world 1!");
    let file_result = read_to_string("abc.txt");
    match file_result {
        Ok(content) => println!("{}", content),
        Err(error) => {
            println!("{:?}", error)
        }
    }
}
