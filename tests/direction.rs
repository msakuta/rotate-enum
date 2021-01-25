use rotate_enum::RotateEnum;

#[derive(RotateEnum, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn main() {
    let up = Direction::Up;
    let left = Direction::Left;
    let down = Direction::Down;
    let right = Direction::Right;

    assert!(up.next() == left);
    assert!(left.next() == down);
    assert!(down.next() == right);
    assert!(right.next() == up);

    assert!(up.prev() == right);
    assert!(left.prev() == up);
    assert!(down.prev() == left);
    assert!(right.prev() == down);
}
