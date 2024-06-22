//! tools to help validate correct use of `unchecked` functions.
//!
//! these `kinda_unchecked` functions will use equivalent implementations that panic when
//! invariants are violated when the `debug_assertions` config is present, but use the
//! corresponding `*_unchecked` otherwise.
//!
//! for example, `GetSaferUnchecked` uses a normal index when debug assertions are enabled, but
//! `.get_unchecked()` otherwise. this means that tests and even fuzzing can be made to exercise
//! panic-on-error cases as desired.

use core::slice::SliceIndex;

pub trait GetSaferUnchecked<T> {
    unsafe fn get_kinda_unchecked<I>(&self, index: I) -> &<I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>;
}

impl<T> GetSaferUnchecked<T> for [T] {
    #[inline(always)]
    unsafe fn get_kinda_unchecked<I>(&self, index: I) -> &<I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>,
    {
        if cfg!(debug_assertions) {
            &self[index]
        } else {
            self.get_unchecked(index)
        }
    }
}

#[inline(always)]
pub unsafe fn unreachable_kinda_unchecked() -> ! {
    if cfg!(debug_assertions) {
        panic!("UB: Unreachable unchecked was executed")
    } else {
        core::hint::unreachable_unchecked()
    }
}
