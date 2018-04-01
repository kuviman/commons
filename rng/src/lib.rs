//! Random number generation.

#![deny(warnings)]

extern crate prelude;
extern crate rand;

#[doc(no_inline)]
pub use rand::{thread_rng, Rng, distributions::{Distribution, Uniform}};

pub(crate) use prelude::*;

/// Generate a random value.
pub fn gen<T>() -> T
where
    Uniform: Distribution<T>,
{
    thread_rng().gen()
}

/// Generate a random value from a given range.
pub fn gen_range<T: PartialOrd + rand::distributions::range::SampleRange>(range: Range<T>) -> T {
    thread_rng().gen_range(range.start, range.end)
}

/// Randomly shuffle an array.
pub fn shuffle<T>(values: &mut [T]) {
    thread_rng().shuffle(values);
}

/// Choose a random element from an array.
///
/// # Panics
///
/// Panics if values is an empty slice.
pub fn choose<T>(values: &[T]) -> &T {
    thread_rng()
        .choose(values)
        .expect("Can not choose from an empty slice")
}
