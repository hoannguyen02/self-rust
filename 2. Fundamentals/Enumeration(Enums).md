* Data that can be one of multiple different possibilities 
* Examples
```
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn which_way(go: Direction) {
    match go {
        Direction::Up => "Up",
        Direction::Down => "Down",
        Direction::Left => "Left",
        Direction::Right => "Right",
    }
}
```