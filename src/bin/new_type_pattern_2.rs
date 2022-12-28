#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    White,
    Yellow
}

#[derive(Debug)]
struct ShirtColor(Color);
impl ShirtColor {
    fn new(color: Color) -> Result<Self, String> {
        match color {
            Color::Purple =>  Err("Purple not allow".to_owned()),
            other => Ok(Self(other))
        }
    }   
}
#[derive(Debug)]
struct ShoesColor(Color);
impl ShoesColor {
    fn new(color: Color) -> Self {
        Self(color)
    }   
}
#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }   
}

fn print_shirt_color(color: ShirtColor) {
    println!("Shirt color = {:?}", color);
}
fn print_shoes_color(color: ShoesColor) {
    println!("Shirt color = {:?}", color);
}
fn print_pants_color(color: PantsColor) {
    println!("Shirt color = {:?}", color);
}

fn main() {
    let shirt_color = ShirtColor::new(Color::Gray);
    match shirt_color {
        Ok(color) => print_shirt_color(color),
        Err(e) => println!("{e}")
    }
    ;
    let pants_color = PantsColor::new(Color::Blue);
    print_pants_color(pants_color);
    let shoes_color = ShoesColor::new(Color::White);
    print_shoes_color(shoes_color);
}