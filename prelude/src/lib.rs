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
pub use rand::{random, thread_rng, Rng};

pub fn random_range<T: PartialOrd + rand::distributions::range::SampleRange>(low: T, high: T) -> T {
    thread_rng().gen_range(low, high)
}

pub fn random_shuffle<T>(values: &mut [T]) {
    thread_rng().shuffle(values);
}

/// Chooses random element
///
/// # Panics
///
/// Panics if values is an empty slice
pub fn random_choose<T>(values: &[T]) -> &T {
    thread_rng()
        .choose(values)
        .expect("Can not choose from an empty slice")
}

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

pub fn default<T: Default>() -> T {
    T::default()
}
