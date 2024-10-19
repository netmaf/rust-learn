// tuple structs

/*
A tuple struct is a struct that has unnamed fields and is defined using parentheses rather than curly braces.
*/

struct Person {
    name: String,
    age: u8,
}

struct Distance(u8, u8, u8);

fn main() {
    let p1 = Person {
        name: String::from("Sunny"),
        age: 20
    };

    //access person fields
    println!("Person {} {}", p1.name, p1.age);

    let distances = Distance(1, 2, 3);

    let sum = distances.0 + distances.1 + distances.2;

    println!("Sum of distances is {}", sum)
}
