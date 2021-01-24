/// # rotate-enum crate
/// 
/// A simple macro that implements `prev()` and `next()` methods to an enum in Rust
/// 
/// ## Motivation
/// 
/// Sometimes you define an enum like this
/// 
/// ```
/// enum Direction {
///     Up,
///     Left,
///     Down,
///     Right,
/// }
/// ```
/// 
/// and you want to rotate them in some logic,
/// 
/// ```
/// # use rotate_enum::RotateEnum;
/// # #[derive(RotateEnum, PartialEq, Clone, Copy)]
/// # enum Direction {
/// #     Up,
/// #     Left,
/// #     Down,
/// #     Right,
/// # }
/// let up = Direction::Up;
/// let left = Direction::Left;
/// let down = Direction::Down;
/// let right = Direction::Right;
/// 
/// assert!(up.next() == left);
/// assert!(left.next() == down);
/// assert!(down.next() == right);
/// assert!(right.next() == up);
/// 
/// assert!(up.prev() == right);
/// assert!(left.prev() == up);
/// assert!(down.prev() == left);
/// assert!(right.prev() == down);
/// ```
/// 
/// You can of course implement these methods manually, but it's repetitive and error prone.
/// Don't you think it should be automated?
/// This crate provides a `RotateEnum` derive macro to just do this.
/// 
/// 
/// ## Usage
/// 
/// ```rust
/// use rotate_enum::RotateEnum;
/// 
/// #[derive(RotateEnum)]
/// enum Direction {
///     Up,
///     Left,
///     Down,
///     Right,
/// }
/// ```
/// 



use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(RotateEnum)]
pub fn rotate_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(data) = &input.data {
        data.variants.iter().map(|v| v).collect::<Vec<_>>()
    } else {
        panic!("derive(RotateEnum) must be applied to an enum");
    };

    let nexts = variants
        .iter()
        .skip(1)
        .chain(variants.iter().take(1))
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
                    #(Self::#nexts => Self::#variants,)*
                }
            }
        }
    };

    tokens.into()
}
