fn main() {
    let rect = Rectangle {
        width: dbg!(15 * 2),
        height: 50,
    };

    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("Can rect hold rect1? {}", rect.can_hold(&rect1));

    println!("rect is: {:#?}", rect);

    println!("square is: {:#?}", Rectangle::square(10));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size,
        };
    }
}
