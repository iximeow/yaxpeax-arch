use core::hash::Hash;

use core::fmt::{self, Debug, Display, Formatter};

use core::ops::{Add, Sub, AddAssign, SubAssign};

use num_traits::identities;
use num_traits::{Bounded, WrappingAdd, WrappingSub, CheckedAdd, CheckedSub};

#[cfg(feature="use-serde")]
use serde::{Deserialize, Serialize};

pub trait AddressBase where Self:
    Debug + Display + AddressDisplay +
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
    fn show(&self, f: &mut Formatter) -> fmt::Result;
}

impl AddressDisplay for usize {
    fn show(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

impl AddressDisplay for u64 {
    fn show(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

impl AddressDisplay for u32 {
    fn show(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

impl AddressDisplay for u16 {
    fn show(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#x}", self)
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
