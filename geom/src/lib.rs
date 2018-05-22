//! Linear algebra and stuff (vectors and matrices).

#![deny(warnings)]

extern crate prelude;

pub(crate) use prelude::*;

mod mat;
mod rect;
mod vec;

pub use self::mat::*;
pub use self::rect::*;
pub use self::vec::*;
