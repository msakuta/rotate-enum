//! # rotate-enum crate
//!
//! This crate provides simple macros that implements `prev()` and `next()` methods to an enum.
//!
//! ## Motivation
//!
//! Sometimes you define an enum like this
//!
//! ```
//! enum Direction {
//!     Up,
//!     Left,
//!     Down,
//!     Right,
//! }
//! ```
//!
//! and you want to rotate them in some logic,
//!
//! ```
//! # use rotate_enum::RotateEnum;
//! # #[derive(RotateEnum, PartialEq, Clone, Copy)]
//! # enum Direction {
//! #     Up,
//! #     Left,
//! #     Down,
//! #     Right,
//! # }
//! let up = Direction::Up;
//! let left = Direction::Left;
//! let down = Direction::Down;
//! let right = Direction::Right;
//!
//! assert!(up.next() == left);
//! assert!(left.next() == down);
//! assert!(down.next() == right);
//! assert!(right.next() == up);
//!
//! assert!(up.prev() == right);
//! assert!(left.prev() == up);
//! assert!(down.prev() == left);
//! assert!(right.prev() == down);
//! ```
//!
//! You can of course implement these methods manually, but it's repetitive and error prone.
//! Don't you think it should be automated?
//! This crate provides a `RotateEnum` derive macro to just do this.
//!
//! ## Shifting
//!
//! This crate also provides [`ShiftEnum`], which will exhaust at the end of the enum list,
//! rather than rotating.
//!
//! ```
//! # use rotate_enum::ShiftEnum;
//! # #[derive(ShiftEnum, PartialEq, Clone, Copy)]
//! # enum Direction {
//! #     Up,
//! #     Left,
//! #     Down,
//! #     Right,
//! # }
//! let up = Direction::Up;
//! let left = Direction::Left;
//! let down = Direction::Down;
//! let right = Direction::Right;
//!
//! assert!(up.next() == Some(left));
//! assert!(left.next() == Some(down));
//! assert!(down.next() == Some(right));
//! assert!(right.next() == None);
//!
//! assert!(up.prev() == None);
//! assert!(left.prev() == Some(up));
//! assert!(down.prev() == Some(left));
//! assert!(right.prev() == Some(down));
//! ```
//!
//! Note that you can only derive either one of `RotateEnum` or `ShiftEnum`, but not both, because their semantics conflict.
//!
//! ## Iterating
//!
//! This crate also provides [`IterEnum`], which will implement [`Iterator`] object
//! that yields enum variants in sequence. The first yield result will be the same
//! variant as the one started the iterator, i.e. `Direction::Up.iter().next() == Some(Direction::Up)`.
//!
//! ```
//! # use rotate_enum::IterEnum;
//! # #[derive(IterEnum, PartialEq, Clone, Copy, Debug)]
//! # enum Direction {
//! #     Up,
//! #     Left,
//! #     Down,
//! #     Right,
//! # }
//! let up = Direction::Up;
//! let left = Direction::Left;
//! let down = Direction::Down;
//! let right = Direction::Right;
//!
//! let mut iter = up.iter();
//! assert!(iter.next() == Some(up));
//! assert!(iter.next() == Some(left));
//! assert!(iter.next() == Some(down));
//! assert!(iter.next() == Some(right));
//! assert!(iter.next() == None);
//!
//! assert_eq!(up.iter().collect::<Vec<_>>(), vec![up, left, down, right]);
//! ```
//!
//! Or, you could start from `"YourEnum"Iterator::new()`.
//!
//! ```
//! # use rotate_enum::IterEnum;
//! # #[derive(IterEnum, PartialEq, Clone, Copy, Debug)]
//! # enum Direction {
//! #     Up,
//! #     Left,
//! #     Down,
//! #     Right,
//! # }
//! assert_eq!(DirectionIterator::new().collect::<Vec<_>>(), vec![
//!     Direction::Up, Direction::Left, Direction::Down, Direction::Right,
//! ]);
//! ```
//!
//! Note that it is not the same as `ShiftEnum` in the sense that the iterator is one-directional, which means you can go only forward and not `prev()`.
//! It can also be used with iterator methods like `collect()`.
//!
//!
//! `IterEnum` also requires deriving `Clone`.
//!
//!
//! ## Usage
//!
//! Use `#[derive(...)]` macro to annotate your enum.
//!
//! ```rust
//! use rotate_enum::RotateEnum;
//!
//! #[derive(RotateEnum)]
//! enum Direction {
//!     Up,
//!     Left,
//!     Down,
//!     Right,
//! }
//! ```
//!
//!
//! ## Note
//!
//! These macros seem trivial, but it's only possible with procedural macros!

use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// This derive macro will implement `next()` and `prev()` methods that rotates
/// the variant to the annotated enum.
///
/// For code examples, see [module-level docs](index.html).
///
/// # Requirements
///
/// * It must be applied to an enum. Structs are not supported or won't make sense.
/// * Enums with any associated data are not supported.
///
/// # Generated methods
///
/// For example, this macro will implement functions like below for
/// `enum Direction`.
///
/// ```
/// # enum Direction {
/// #     Up,
/// #     Left,
/// #     Down,
/// #     Right,
/// # }
/// impl Direction {
///     fn next(self) -> Self {
///         match self {
///             Self::Up => Self::Left,
///             Self::Left => Self::Down,
///             Self::Down => Self::Right,
///             Self::Right => Self::Up,
///         }
///     }
///
///     fn prev(self) -> Self {
///         match self {
///             Self::Up => Self::Right,
///             Self::Left => Self::Up,
///             Self::Down => Self::Left,
///             Self::Right => Self::Down,
///         }
///     }
/// }
/// ```
#[proc_macro_derive(RotateEnum)]
pub fn rotate_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(data) = &input.data {
        data.variants.iter().collect::<Vec<_>>()
    } else {
        panic!("derive(RotateEnum) must be applied to an enum");
    };

    let nexts = variants
        .iter()
        .skip(1)
        .chain(variants.get(0))
        .map(|v| (&v.ident))
        .collect::<Vec<_>>();

    let tokens = quote! {
        impl #name{
            pub fn next(self) -> Self {
                match self {
                    #(Self::#variants => Self::#nexts, )*
                }
            }
            pub fn prev(self) -> Self {
                match self {
                    #(Self::#nexts => Self::#variants, )*
                }
            }
        }
    };

    tokens.into()
}

/// This derive macro will implement `next()` and `prev()` methods that shifts
/// the variant to the annotated enum.
///
/// * `next()` will return `Some(Variant)` where `Variant` is next one in the enum, or `None` if it was the last variant of the enum.
/// * `prev()` will return `Some(Variant)` where `Variant` is previous one in the enum, or `None` if it was the first variant of the enum.
///
/// For code examples, see [module-level docs](index.html).
///
/// # Requirements
///
/// * It must be applied to an enum. Structs are not supported or won't make sense.
/// * Enums with any associated data are not supported.
///
/// # Generated methods
///
/// For example, this macro will implement functions like below for
/// `enum Direction`.
///
/// ```
/// # enum Direction {
/// #     Up,
/// #     Left,
/// #     Down,
/// #     Right,
/// # }
/// impl Direction {
///     fn next(self) -> Option<Self> {
///         match self {
///             Self::Up => Some(Self::Left),
///             Self::Left => Some(Self::Down),
///             Self::Down => Some(Self::Right),
///             Self::Right => None,
///         }
///     }
///
///     fn prev(self) -> Option<Self> {
///         match self {
///             Self::Up => None,
///             Self::Left => Some(Self::Up),
///             Self::Down => Some(Self::Left),
///             Self::Right => Some(Self::Down),
///         }
///     }
/// }
/// ```
#[proc_macro_derive(ShiftEnum)]
pub fn shift_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(data) = &input.data {
        data.variants.iter().collect::<Vec<_>>()
    } else {
        panic!("derive(RotateEnum) must be applied to an enum");
    };

    let nexts = variants
        .iter()
        .skip(1)
        .map(|v| quote! { Some(Self::#v) })
        .chain(Some(quote! { None }))
        .collect::<Vec<_>>();

    let none_quote = Some(quote! { None });
    let prevs = variants
        .iter()
        .take(variants.len() - 1)
        .map(|v| quote! { Some(Self::#v) })
        .collect::<Vec<_>>();

    let prevs = none_quote.iter().chain(&prevs).collect::<Vec<_>>();

    let tokens = quote! {
        impl #name{
            pub fn next(self) -> Option<Self> {
                match self {
                    #(Self::#variants => #nexts, )*
                }
            }
            pub fn prev(self) -> Option<Self> {
                match self {
                    #(Self::#variants => #prevs, )*
                }
            }
        }
    };

    tokens.into()
}

/// This derive macro will implement `iter()` method to the annotated enum that sequentially
/// yield the variant of the enum.
///
/// For code examples, see [module-level docs](index.html).
///
/// # Requirements
///
/// * It must be applied to an enum. Structs are not supported or won't make sense.
/// * Enums with any associated data are not supported.
/// * Enum also needs to derive [`Clone`].
///
/// # Generated methods
///
/// For example, this macro will implement an iterator and methods like below for
/// `enum Direction`.
///
/// ```
/// # #[derive(Clone, Debug)]
/// # enum Direction {
/// #     Up,
/// #     Left,
/// #     Down,
/// #     Right,
/// # }
/// struct DirectionIterator(Option<Direction>);
///
/// impl Iterator for DirectionIterator {
///     type Item = Direction;
///     fn next(&mut self) -> Option<Self::Item> {
///         let ret = self.0.clone();
///         self.0 = match self.0 {
///             Some(Direction::Up) => Some(Direction::Left),
///             Some(Direction::Left) => Some(Direction::Down),
///             Some(Direction::Down) => Some(Direction::Right),
///             Some(Direction::Right) => None,
///             None => None,
///         };
///         ret
///     }
/// }
/// ```
#[proc_macro_derive(IterEnum)]
pub fn iter_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(data) = &input.data {
        data.variants.iter().collect::<Vec<_>>()
    } else {
        panic!("derive(RotateEnum) must be applied to an enum");
    };

    let first_variant = variants
        .first()
        .expect("derive(IterEnum) expects at least one variant in enum");

    let nexts = variants
        .iter()
        .skip(1)
        .map(|v| quote! { Some(#name::#v) })
        .chain(Some(quote! { None }))
        .collect::<Vec<_>>();

    let iterator_name = syn::Ident::new(&(name.to_string() + "Iterator"), name.span());

    let tokens = quote! {

        struct #iterator_name(Option<#name>);

        impl #iterator_name {
            fn new() -> Self {
                Self(Some(#name::#first_variant))
            }
        }

        impl Iterator for #iterator_name {
            type Item = #name;
            fn next(&mut self) -> Option<Self::Item> {
                let ret = self.0.clone();
                self.0 = match self.0 {
                    #(Some(#name::#variants) => #nexts, )*
                    None => None,
                };
                ret
            }
        }

        impl #name {
            fn iter(&self) -> #iterator_name {
                #iterator_name(Some(self.clone()))
            }
        }
    };

    tokens.into()
}
