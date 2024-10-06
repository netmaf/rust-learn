

trait Summary {
    fn summarize(&self) -> String {
        return String::from("User is x years old");
    }

    fn st() -> String {
        return String::from("Static fun");
    }
} 

struct User {
    name: String,
    age: u8
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User {} is {} years old", self.name, self.age);
    }
}

fn notify(u: &impl Summary) { // this functions only accepts arge which implement summary
    println!("Inside notifire {}", u.summarize())
}

// notify impl is syntactic suger -> under the hood it does the following
fn notify2<T: Summary>(item: T) {
    println!("Inside notifire {}", item.summarize())
}

// fn notify2<T: Summary + Fix>(item: &T) { //impl multiple traits
//     println!("Inside notifire {}", item.summarize())
// }

fn main() {
    let sunny = User {
        name: String::from("Sunny"),
        age: 10
    };

    println!("{}", User::st());
    println!("{}", sunny.summarize());

    notify(&sunny);
}
