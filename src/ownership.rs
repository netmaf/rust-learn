fn main() {
    let mut s1 = String::from("Sunny");
    s1 = fun(s1);
    println!("{}", s1);

    fun2(s1);
}

// move ownership + return back
fn fun(s1: String) -> String {
    return s1;
}

// moving ownerhsip
fn fun2(s1: String) {
    println!("{}", s1)
}