use rotate_enum::RotateEnum;

#[derive(RotateEnum, PartialEq, Clone, Copy, Default)]
enum Direction {
    #[default]
    Up,
    Left,
    Down,
    Right,
}

/// Tests an enum with Default annotations
#[test]
fn test_rotate_with_annotation() {
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
