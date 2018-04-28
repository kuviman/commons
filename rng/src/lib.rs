//! Random number generation.

#![deny(warnings)]

extern crate prelude;
extern crate rand;
#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
#[macro_use]
extern crate stdweb;

#[doc(no_inline)]
pub use rand::{Rng, distributions::{Distribution, Standard}};

pub(crate) use prelude::*;

#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
mod web_rng {
    use ::*;

    fn global_rng() -> &'static mut rand::StdRng {
        static mut GLOBAL_RNG: Option<rand::StdRng> = None;
        unsafe {
            if GLOBAL_RNG.is_none() {
                use stdweb::unstable::TryInto;
                fn gen_byte() -> u8 {
                    let x: f64 = js!{ return Math.random(); }.try_into().unwrap();
                    clamp(x * 256.0, 0.0, 255.0) as u8
                }
                let mut seed: [u8; 32] = mem::uninitialized();
                for x in &mut seed {
                    *x = gen_byte();
                }
                GLOBAL_RNG = Some(rand::SeedableRng::from_seed(seed));
            }
            GLOBAL_RNG.as_mut().unwrap()
        }
    }

    pub struct StdRng;

    impl rand::RngCore for StdRng {
        fn next_u32(&mut self) -> u32 {
            global_rng().next_u32()
        }
        fn next_u64(&mut self) -> u64 {
            global_rng().next_u64()
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            global_rng().fill_bytes(dest)
        }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            global_rng().try_fill_bytes(dest)
        }
    }
}

#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
pub fn default() -> web_rng::StdRng {
    web_rng::StdRng
}
#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
pub fn default() -> rand::ThreadRng {
    rand::thread_rng()
}

/// Generate a random value.
pub fn gen<T>() -> T
where
    Standard: Distribution<T>,
{
    default().gen()
}

/// Generate a random value from a given range.
pub fn gen_range<T: PartialOrd + rand::distributions::range::SampleRange>(range: Range<T>) -> T {
    default().gen_range(range.start, range.end)
}

/// Randomly shuffle an array.
pub fn shuffle<T>(values: &mut [T]) {
    default().shuffle(values);
}

/// Choose a random element from an array.
///
/// # Panics
///
/// Panics if values is an empty slice.
pub fn choose<T>(values: &[T]) -> &T {
    default()
        .choose(values)
        .expect("Can not choose from an empty slice")
}

#[test]
fn test_working() {
    gen::<i8>();
    gen::<i16>();
    gen::<i32>();
    gen::<i64>();
    gen::<isize>();

    gen::<u8>();
    gen::<u16>();
    gen::<u32>();
    gen::<u64>();
    gen::<usize>();

    gen::<f32>();
    gen::<f64>();
}
