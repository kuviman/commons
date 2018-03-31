#![deny(warnings)]

extern crate prelude;

pub(crate) use prelude::*;

mod vec;
mod mat;
mod rect;

pub use self::vec::*;
pub use self::mat::*;
pub use self::rect::*;
