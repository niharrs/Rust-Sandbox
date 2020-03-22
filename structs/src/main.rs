use std::io;

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    fn area (&self) -> i32 {
        self.width * self.height
    }
}

fn main() {
    let mut width_str = String::new();
    let mut height_str = String::new();

    println! ("Please enter the width.");
    io::stdin().read_line(&mut width_str).expect("Couldn't read the vaule");

    println! ("Please enter the height.");
    io::stdin().read_line(&mut height_str).expect("Couldn't read the vaule");

    let width : i32 = width_str.trim().parse().expect("Couldn't read the value");
    let height : i32 = height_str.trim().parse().expect("Couldn't read the value");

    let rect1 = Rectangle {
        width,
        height
    };
    
    println! ("Area of the rectange is {}", rect1.area());
}
