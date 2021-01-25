![Rust](https://github.com/msakuta/rotate-enum/workflows/Rust/badge.svg)

# rotate-enum crate

Simple derive macros that implement `prev()` and `next()` methods to an enum in Rust

## Motivation

Sometimes you define an enum like this

```rust
enum Direction {
    Up,
    Left,
    Down,
    Right,
}
```

and you want to rotate them in some logic,

```rust
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
```

You can of course implement these methods manually, but it's repetitive and error prone.
Don't you think it should be automated?
This crate provides a `RotateEnum` derive macro to just do this.


## Shifting

This crate also provides `ShiftEnum`, which will exhaust at the end of the enum list,
rather than rotating.

```rust
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
```

## Usage

```rust
use rotate_enum::RotateEnum;

#[derive(RotateEnum)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}
```
