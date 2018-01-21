#![deny(warnings)]

pub extern crate num;
pub extern crate owning_ref;
pub extern crate rand;

#[doc(no_inline)]
pub use std::rc::Rc;
#[doc(no_inline)]
pub use std::cell::{Cell, Ref, RefCell, RefMut};
#[doc(no_inline)]
pub use std::marker::PhantomData;
#[doc(no_inline)]
pub use std::os::raw::{c_char, c_double, c_float, c_int, c_long, c_short, c_ulong, c_ushort,
                       c_void};
#[doc(no_inline)]
pub use std::ffi::{CStr, CString};
#[doc(no_inline)]
pub use std::borrow::Cow;
#[doc(no_inline)]
pub use std::ops::{Deref, DerefMut, Range, RangeFrom, RangeFull, RangeTo};
#[doc(no_inline)]
pub use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
#[doc(no_inline)]
pub use std::ops::{Index, IndexMut};
#[doc(no_inline)]
pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
#[doc(no_inline)]
pub use std::fmt::{Debug, Display, Formatter};
#[doc(no_inline)]
pub use std::sync::{Arc, Mutex};
#[doc(no_inline)]
pub use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
#[doc(no_inline)]
pub use std::mem;
#[doc(no_inline)]
pub use std::thread;

#[doc(no_inline)]
pub use num::{clamp, Float, Integer, Num};

#[doc(no_inline)]
pub use owning_ref::{OwningHandle, OwningRef, OwningRefMut};

#[doc(no_inline)]
pub use rand::Rng;

mod color;
mod algebra;
mod range;
mod timer;
mod acell;
mod stable_fn;

pub use color::*;
pub use algebra::*;
pub use range::*;
pub use timer::*;
pub use acell::*;
pub use stable_fn::*;

pub fn thread_rng() -> Box<Rng> {
    #[cfg(target_os = "emscripten")]
    {
        extern "C" {
            fn emscripten_random() -> c_float;
        }
        struct EmscriptenRng;
        impl Rng for EmscriptenRng {
            fn next_u32(&mut self) -> u32 {
                unsafe { (emscripten_random() as f64 * std::u32::MAX as f64) as u32 }
            }
            fn next_f32(&mut self) -> f32 {
                unsafe { emscripten_random() as f32 }
            }
            fn next_f64(&mut self) -> f64 {
                unsafe { emscripten_random() as f64 }
            }
        }
        Box::new(EmscriptenRng)
    }
    #[cfg(not(target_os = "emscripten"))]
    Box::new(rand::thread_rng())
}

pub fn random<R: rand::Rand>() -> R {
    R::rand(&mut thread_rng())
}

pub fn default<T: Default>() -> T {
    T::default()
}
