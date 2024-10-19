// tuple enums and normal enums

enum PaymentMethod {
    CreditCard {
        number: String,
        exp_month: u8,
        exp_year: u16
    },
    PayPal {
        email: String
    },
    Cash
}

enum Shape {
    Circle(u8),
    Square(u8, u8)
}



fn main() {
    let payment = PaymentMethod::PayPal { email: String::from("lokeshsw2@gmail.com") };

    match payment {
        PaymentMethod::PayPal { email } => {
            println!("Consumer email {}", email)
        },
        PaymentMethod::Cash => {
            println!("Processing cash payment")
        },
        PaymentMethod::CreditCard { number, exp_month, exp_year } => {
            println!("{number} {exp_month} {exp_year}")
        }
    }

    // tuple enums
    let shape = Shape::Square(10, 10);

    match shape {
        Shape::Circle(rad) => {
            println!("circle radius {rad}")
        },
        Shape::Square(l, h) => {
            let area = l * h;
            println!("area ->  {area}")
        }
    }
}
