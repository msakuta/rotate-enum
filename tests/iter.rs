use rotate_enum::IterEnum;

#[derive(IterEnum, PartialEq, Clone, Copy, Debug)]
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

    let mut iter = up.iter();
    assert!(iter.next() == Some(up));
    assert!(iter.next() == Some(left));
    assert!(iter.next() == Some(down));
    assert!(iter.next() == Some(right));
    assert!(iter.next() == None);

    assert_eq!(up.iter().collect::<Vec<_>>(), vec![up, left, down, right]);
}
