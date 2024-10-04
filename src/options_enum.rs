
fn find_first_a(str: String) -> Option<i32> {
    for (index, char) in str.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }

    return None;

}

fn main() {
    println!("Hello, world 1!");
    let res: Option<i32> = find_first_a(String::from("Sunnaunn"));
    match res {
        Some(a) => {
            println!("{}", a)
        },
        None => {
            println!("Value didnt match")
        }
    }

}
