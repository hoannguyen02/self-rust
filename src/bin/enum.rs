enum Direction {
    Up,
    Down,
    Left,
    Right
}

enum Color {
    Red,
    Yellow,
    Blue
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Yellow => println!("Yellow"),
        Color::Blue => println!("Blue")
    }
}

fn main() {
    let left = Direction::Left;
    let result =  match left {
        Direction::Up => "Up",
        Direction::Down => "Down",
        Direction::Left => "Left",
        Direction::Right => "Right",
    };

    println!("{result}");

    print_color(Color::Blue);
}