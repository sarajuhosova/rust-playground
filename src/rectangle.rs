struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(side: u32) -> Rectangle {
        Rectangle::new(side, side)
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn main() {
    let rect = Rectangle::new(13, 21);
    println!("{}", rect.area());

    let square = Rectangle::square(56);
    println!("{}", square.area());
}
