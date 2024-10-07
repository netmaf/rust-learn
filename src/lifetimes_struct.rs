
struct User<'a> {
    name: &'a str
}

fn main() {
    let user1_name = String::from("Sunny");
    let mut user;
    {
        let user2_name = String::from("Sunny");
        user = User {
            name: &user1_name
        };

        // user.name = &user2_name; // compiler complains here
    }
    println!("{}", user.name)

}
