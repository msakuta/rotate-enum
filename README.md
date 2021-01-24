# rotate-enum crate

A simple macro that implements `prev()` and `next()` methods to an enum in Rust

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
