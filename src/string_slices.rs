// String slices

fn main() {
    let word: String = String::from("Sunny Sun");

    let word2 = "abc"; // this is also a string slice -> points word in a binary

    let ans = get_first_word(&word);
    println!("{}", ans)

    // str.push_str(" S"); // append to string
    // println!("{}", str);

    // str.replace_range(0..1, "");
    // println!("{}", str); 
}

fn get_first_word(word: &String) -> &str {
    let mut i = 0;
    for ch in word.chars() {
        if ch == ' ' {
            break;
        }
        i = i + 1;
    }

    // return &str[0..2];
    return &word[0..i];
}