use rotate_enum::ShiftEnum;

#[derive(ShiftEnum, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[test]
fn test_shift() {
    let up = Direction::Up;
    let left = Direction::Left;
    let down = Direction::Down;
    let right = Direction::Right;

    assert!(up.next() == Some(left));
    assert!(left.next() == Some(down));
    assert!(down.next() == Some(right));
    assert!(right.next() == None);

    assert!(up.prev() == None);
    assert!(left.prev() == Some(up));
    assert!(down.prev() == Some(left));
    assert!(right.prev() == Some(down));
}
