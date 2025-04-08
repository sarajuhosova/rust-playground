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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle() {
        let rect = Rectangle::new(13, 21);
        assert_eq!(rect.area(), 273);
    }

    #[test]
    fn test_square() {
        let square = Rectangle::square(56);
        assert_eq!(square.area(), 3136);
    }
}
