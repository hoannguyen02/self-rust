struct User  {
    name: String,
    age: i32
}

impl User {
    fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }
    fn display_name(&self) {
        println!("{:?}", self.name)
    }
    fn display_age(&self) {
        println!("{:?}", self.age)
    }
}

enum Color {
    Red,
    Green
}
impl Color {
    fn print(&self) {
        match self {
            Color::Green => println!("Color: Green"), 
            Color::Red => println!("Color: Red"), 
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64
}

impl Dimensions {
    fn new(width: f64, height: f64, depth: f64) -> Self {
        Self {
            width,
            height,
            depth
        }
    }
    fn print(&self) {
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
        println!("Depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    weight: f64,
    color: Color,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self { weight, color, dimensions }
    } 

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight {:?}", self.weight);
    }
}


fn main() {
    // User
    let me = User::new("Hoan".to_owned(), 32);
    me.display_name();
    me.display_age();


    // Shipping box
    let dimensions =  Dimensions::new(3.0, 4.0, 5.0);
    let my_box = ShippingBox::new(5.0, Color::Green, dimensions);
    my_box.print();
}