#![deny(warnings)]

extern crate prelude;
extern crate rand;

#[doc(no_inline)]
pub use rand::{thread_rng, Rng, distributions::{Distribution, Uniform}};

pub(crate) use prelude::*;

pub fn gen<T>() -> T
where
    Uniform: Distribution<T>,
{
    thread_rng().gen()
}

pub fn gen_range<T: PartialOrd + rand::distributions::range::SampleRange>(range: Range<T>) -> T {
    thread_rng().gen_range(range.start, range.end)
}

pub fn shuffle<T>(values: &mut [T]) {
    thread_rng().shuffle(values);
}

/// Chooses random element
///
/// # Panics
///
/// Panics if values is an empty slice
pub fn choose<T>(values: &[T]) -> &T {
    thread_rng()
        .choose(values)
        .expect("Can not choose from an empty slice")
}
