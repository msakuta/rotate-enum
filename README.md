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

Note that you can only derive either one of `RotateEnum` or `ShiftEnum`, but not both, because their semantics conflict.

## Iterating

This crate also provides `IterEnum`, which will implement `Iterator` object
that yields enum variants in sequence. The first yield result will be the same
variant as the one started the iterator, i.e. `Direction::Up.iter().next() == Some(Direction::Up)`.

```rust
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
```

Note that it is not the same as `ShiftEnum` in the sense that the iterator is one-directional, which means you can go only forward and not `prev()`.
It can also be used with iterator methods like `collect()`.

`IterEnum` also requires deriving `Clone`.

## Usage

Use `#[derive(...)]` macro to annotate your enum.

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
