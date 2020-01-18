use core::hash::Hash;

use core::fmt;

use core::ops::{Add, Sub, AddAssign, SubAssign};

use num_traits::identities;
use num_traits::{Bounded, WrappingAdd, WrappingSub, CheckedAdd, CheckedSub};

#[cfg(feature="use-serde")]
use serde::{Deserialize, Serialize};

pub trait AddressBase where Self:
    AddressDisplay +
    Copy + Clone + Sized + Hash +
    Ord + Eq + PartialEq + Bounded +
    Add<Output=Self> + Sub<Output=Self> +
    AddAssign + SubAssign +
    WrappingAdd + WrappingSub +
    CheckedAdd + CheckedSub +
    Hash +
    identities::One + identities::Zero {
    fn to_linear(&self) -> usize;
}

#[cfg(all(feature="use-serde", feature="address-parse"))]
pub trait Address where Self:
    AddressBase +
    Serialize + for<'de> Deserialize<'de> +
    AddrParse {
}

#[cfg(all(feature="use-serde", not(feature="address-parse")))]
pub trait Address where Self:
    AddressBase +
    Serialize + for<'de> Deserialize<'de> {
}

#[cfg(all(not(feature="use-serde"), feature="address-parse"))]
pub trait Address where Self:
    AddressBase + AddrParse {
}

#[cfg(all(not(feature="use-serde"), not(feature="address-parse")))]
pub trait Address where Self: AddressBase { }

impl AddressBase for u16 {
    fn to_linear(&self) -> usize { *self as usize }
}

impl Address for u16 {}

impl AddressBase for u32 {
    fn to_linear(&self) -> usize { *self as usize }
}

impl Address for u32 {}

impl AddressBase for u64 {
    fn to_linear(&self) -> usize { *self as usize }
}

impl Address for u64 {}

impl AddressBase for usize {
    fn to_linear(&self) -> usize { *self }
}

impl Address for usize {}

pub trait AddressDisplay {
    type Show: fmt::Display;
    fn show(&self) -> Self::Show;
}

impl AddressDisplay for usize {
    type Show = AddressDisplayUsize;

    fn show(&self) -> AddressDisplayUsize {
        AddressDisplayUsize(*self)
    }
}

#[repr(transparent)]
pub struct AddressDisplayUsize(usize);

impl fmt::Display for AddressDisplayUsize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl AddressDisplay for u64 {
    type Show = AddressDisplayU64;

    fn show(&self) -> AddressDisplayU64 {
        AddressDisplayU64(*self)
    }
}

#[repr(transparent)]
pub struct AddressDisplayU64(u64);

impl fmt::Display for AddressDisplayU64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl AddressDisplay for u32 {
    type Show = AddressDisplayU32;

    fn show(&self) -> AddressDisplayU32 {
        AddressDisplayU32(*self)
    }
}

#[repr(transparent)]
pub struct AddressDisplayU32(u32);

impl fmt::Display for AddressDisplayU32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl AddressDisplay for u16 {
    type Show = AddressDisplayU16;

    fn show(&self) -> AddressDisplayU16 {
        AddressDisplayU16(*self)
    }
}

#[repr(transparent)]
pub struct AddressDisplayU16(u16);

impl fmt::Display for AddressDisplayU16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

/*
 * TODO: this should be FromStr.
 * that would require newtyping address primitives, though
 *
 * this is not out of the question, BUT is way more work than
 * i want to put in right now
 *
 * this is one of those "clean it up later" situations
 */
#[cfg(feature="address-parse")]
use core::str::FromStr;

#[cfg(feature="address-parse")]
pub trait AddrParse: Sized {
    type Err;
    fn parse_from(s: &str) -> Result<Self, Self::Err>;
}

#[cfg(feature="address-parse")]
impl AddrParse for usize {
    type Err = core::num::ParseIntError;
    fn parse_from(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            usize::from_str_radix(&s[2..], 16)
        } else {
            usize::from_str(s)
        }
    }
}

#[cfg(feature="address-parse")]
impl AddrParse for u64 {
    type Err = core::num::ParseIntError;
    fn parse_from(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            u64::from_str_radix(&s[2..], 16)
        } else {
            u64::from_str(s)
        }
    }
}

#[cfg(feature="address-parse")]
impl AddrParse for u32 {
    type Err = core::num::ParseIntError;
    fn parse_from(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            u32::from_str_radix(&s[2..], 16)
        } else {
            u32::from_str(s)
        }
    }
}

#[cfg(feature="address-parse")]
impl AddrParse for u16 {
    type Err = core::num::ParseIntError;
    fn parse_from(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            u16::from_str_radix(&s[2..], 16)
        } else {
            u16::from_str(s)
        }
    }
}
