#![deny(warnings)]

extern crate num_integer;
extern crate num_traits;

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
pub use num_traits::{clamp, Float, Num};
#[doc(no_inline)]
pub use num_integer::Integer;

pub fn min_max<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    min_max(a, b).0
}

pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    min_max(a, b).1
}

mod range;
mod acell;
mod stable_fn;

pub use range::*;
pub use acell::*;
pub use stable_fn::*;

pub fn default<T: Default>() -> T {
    T::default()
}
