struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn perimeter(&self) -> u32 {
        2 * (self.height + self.width)
    }

    fn debug() -> u8{
        return 1;
    }
}

fn main() {

   let rect1 = Rect {
    width: 20,
    height: 30
   };

   println!("Area of reactagle is {}", rect1.area());
   println!("Perimeter of reactagle is {}", rect1.perimeter());
   println!("Debug {}", Rect::debug());
}
