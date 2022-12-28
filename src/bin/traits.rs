trait Perimeter {
    fn calc(&self) -> i32;
}

struct Square {
    side: i32

}
impl  Perimeter for Square {
    fn calc(&self) -> i32 {
        self.side * 4
    }
}

struct Triangle {
    side_a: i32,
    side_b: i32,
    side_c: i32,
}
impl Perimeter for Triangle {
    fn calc(&self) -> i32 {
        self.side_a + self.side_b + self.side_c
    }
}

impl  Default for Triangle {
    fn default() -> Self {
        Self { side_a: 2, side_b: 3, side_c: 4 }
    }
}

fn print_perimeter(shape: impl Perimeter) {
    let perimeter = shape.calc();
    println!("Perimeter: {:?}", perimeter);
}

fn main() {
    let square = Square {
        side: 5
    };
    // let triangle = Triangle {
    //     side_a: 2,
    //     side_b: 3,
    //     side_c: 4
    // };
    let triangle = Triangle::default();
    print_perimeter(square);
    print_perimeter(triangle);

}

