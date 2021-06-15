use core::hash::Hash;

use core::fmt;

use core::ops::{Add, Sub, AddAssign, SubAssign};

use num_traits::identities;
use num_traits::{Bounded, WrappingAdd, WrappingSub, CheckedAdd, Zero, One};

#[cfg(feature="use-serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature="use-serde")]
pub trait AddressDiffAmount: Copy + Clone + PartialEq + PartialOrd + Eq + Ord + identities::Zero + identities::One + Serialize + for<'de> Deserialize<'de> {}
#[cfg(not(feature="use-serde"))]
pub trait AddressDiffAmount: Copy + Clone + PartialEq + PartialOrd + Eq + Ord + identities::Zero + identities::One {}

impl AddressDiffAmount for u64 {}
impl AddressDiffAmount for u32 {}
impl AddressDiffAmount for u16 {}
impl AddressDiffAmount for usize {}

/// a struct describing the differece between some pair of `A: Address`. this is primarily useful
/// in describing the size of an instruction, or the relative offset of a branch.
///
/// for any address type `A`, the following must hold:
/// ```rust
/// use yaxpeax_arch::AddressBase;
/// fn diff_check<A: AddressBase + core::fmt::Debug>(left: A, right: A) {
///     let diff = left.diff(&right);
///     if let Some(offset) = diff {
///         assert_eq!(left.wrapping_offset(offset), right);
///     }
/// }
/// ```
///
/// which is to say, `yaxpeax` assumes associativity holds when `diff` yields a `Some`.
#[cfg(feature="use-serde")]
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct AddressDiff<T: AddressBase> {
    // the AddressDiffAmount trait fools `Deserialize`'s proc macro, so we have to explicitly write
    // the bound serde should use.
    #[serde(bound(deserialize = "T::Diff: AddressDiffAmount"))]
    amount: T::Diff,
}
/// a struct describing the differece between some pair of `A: Address`. this is primarily useful
/// in describing the size of an instruction, or the relative offset of a branch.
///
/// for any address type `A`, the following must hold:
/// ```rust
/// use yaxpeax_arch::AddressBase;
/// fn diff_check<A: AddressBase + core::fmt::Debug>(left: A, right: A) {
///     let diff = left.diff(&right);
///     if let Some(offset) = diff {
///         assert_eq!(left.wrapping_offset(offset), right);
///     }
/// }
/// ```
///
/// which is to say, `yaxpeax` assumes associativity holds when `diff` yields a `Some`.
#[cfg(not(feature="use-serde"))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct AddressDiff<T: AddressBase> {
    amount: T::Diff,
}

impl<T: Address> AddressDiff<T> {
    pub fn from_const(amount: T::Diff) -> Self {
        AddressDiff { amount }
    }
}

impl<T: Address> fmt::Debug for AddressDiff<T> where T::Diff: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AddressDiff({:?})", self.amount)
    }
}

impl<T: Address> AddressDiff<T> {
    pub fn one() -> Self {
        AddressDiff {
            amount: <T as AddressBase>::Diff::one(),
        }
    }

    pub fn zero() -> Self {
        AddressDiff {
            amount: <T as AddressBase>::Diff::zero(),
        }
    }
}

impl Sub<AddressDiff<u16>> for u16 {
    type Output = Self;

    fn sub(self, other: AddressDiff<Self>) -> Self::Output {
        self - other.amount
    }
}

impl Sub<AddressDiff<u32>> for u32 {
    type Output = Self;

    fn sub(self, other: AddressDiff<Self>) -> Self::Output {
        self - other.amount
    }
}

impl Sub<AddressDiff<u64>> for u64 {
    type Output = Self;

    fn sub(self, other: AddressDiff<Self>) -> Self::Output {
        self - other.amount
    }
}

impl Sub<AddressDiff<usize>> for usize {
    type Output = Self;

    fn sub(self, other: AddressDiff<Self>) -> Self::Output {
        self - other.amount
    }
}

impl Add<AddressDiff<u16>> for u16 {
    type Output = Self;

    fn add(self, other: AddressDiff<Self>) -> Self::Output {
        self + other.amount
    }
}

impl Add<AddressDiff<u32>> for u32 {
    type Output = Self;

    fn add(self, other: AddressDiff<Self>) -> Self::Output {
        self + other.amount
    }
}

impl Add<AddressDiff<u64>> for u64 {
    type Output = Self;

    fn add(self, other: AddressDiff<Self>) -> Self::Output {
        self + other.amount
    }
}

impl Add<AddressDiff<usize>> for usize {
    type Output = Self;

    fn add(self, other: AddressDiff<Self>) -> Self::Output {
        self + other.amount
    }
}

impl SubAssign<AddressDiff<u16>> for u16 {
    fn sub_assign(&mut self, other: AddressDiff<Self>) {
        *self -= other.amount;
    }
}

impl SubAssign<AddressDiff<u32>> for u32 {
    fn sub_assign(&mut self, other: AddressDiff<Self>) {
        *self -= other.amount;
    }
}

impl SubAssign<AddressDiff<u64>> for u64 {
    fn sub_assign(&mut self, other: AddressDiff<Self>) {
        *self -= other.amount;
    }
}

impl SubAssign<AddressDiff<usize>> for usize {
    fn sub_assign(&mut self, other: AddressDiff<Self>) {
        *self -= other.amount;
    }
}

impl AddAssign<AddressDiff<u16>> for u16 {
    fn add_assign(&mut self, other: AddressDiff<Self>) {
        *self += other.amount;
    }
}

impl AddAssign<AddressDiff<u32>> for u32 {
    fn add_assign(&mut self, other: AddressDiff<Self>) {
        *self += other.amount;
    }
}

impl AddAssign<AddressDiff<u64>> for u64 {
    fn add_assign(&mut self, other: AddressDiff<Self>) {
        *self += other.amount;
    }
}

impl AddAssign<AddressDiff<usize>> for usize {
    fn add_assign(&mut self, other: AddressDiff<Self>) {
        *self += other.amount;
    }
}

pub trait AddressBase where Self:
    AddressDisplay +
    Copy + Clone + Sized + Hash +
    Ord + Eq + PartialEq + Bounded +
    Add<AddressDiff<Self>, Output=Self> + Sub<AddressDiff<Self>, Output=Self> +
    AddAssign<AddressDiff<Self>> + SubAssign<AddressDiff<Self>> +
    identities::Zero +
    Hash {
    type Diff: AddressDiffAmount;
    fn to_linear(&self) -> usize;

    /// compute the `AddressDiff` beetween `self` and `other`.
    ///
    /// may return `None` if the two addresses aren't comparable. for example, if a pair of
    /// addresses are a data-space address and code-space address, there may be no scalar that can
    /// describe the difference between them.
    fn diff(&self, other: &Self) -> Option<AddressDiff<Self>>;
    /*
    {
        Some(AddressDiff { amount: self.wrapping_sub(other) })
    }
    */

    fn wrapping_offset(&self, other: AddressDiff<Self>) -> Self;
    /*
    {
        self.wrapping_add(&other.amount)
    }
    */

    fn checked_offset(&self, other: AddressDiff<Self>) -> Option<Self>;
    /*
    {
        self.checked_add(&other.amount)
    }
    */
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
    type Diff = Self;
    fn to_linear(&self) -> usize { *self as usize }

    fn diff(&self, other: &Self) -> Option<AddressDiff<Self>> {
        Some(AddressDiff { amount: self.wrapping_sub(other) })
    }
    fn wrapping_offset(&self, other: AddressDiff<Self>) -> Self {
        self.wrapping_add(&other.amount)
    }

    fn checked_offset(&self, other: AddressDiff<Self>) -> Option<Self> {
        self.checked_add(&other.amount)
    }
}

impl Address for u16 {}

impl AddressBase for u32 {
    type Diff = Self;
    fn to_linear(&self) -> usize { *self as usize }

    fn diff(&self, other: &Self) -> Option<AddressDiff<Self>> {
        Some(AddressDiff { amount: self.wrapping_sub(other) })
    }
    fn wrapping_offset(&self, other: AddressDiff<Self>) -> Self {
        self.wrapping_add(&other.amount)
    }

    fn checked_offset(&self, other: AddressDiff<Self>) -> Option<Self> {
        self.checked_add(&other.amount)
    }
}

impl Address for u32 {}

impl AddressBase for u64 {
    type Diff = Self;
    fn to_linear(&self) -> usize { *self as usize }

    fn diff(&self, other: &Self) -> Option<AddressDiff<Self>> {
        Some(AddressDiff { amount: self.wrapping_sub(other) })
    }
    fn wrapping_offset(&self, other: AddressDiff<Self>) -> Self {
        self.wrapping_add(&other.amount)
    }

    fn checked_offset(&self, other: AddressDiff<Self>) -> Option<Self> {
        self.checked_add(&other.amount)
    }
}

impl Address for u64 {}

impl AddressBase for usize {
    type Diff = Self;
    fn to_linear(&self) -> usize { *self }

    fn diff(&self, other: &Self) -> Option<AddressDiff<Self>> {
        Some(AddressDiff { amount: self.wrapping_sub(other) })
    }
    fn wrapping_offset(&self, other: AddressDiff<Self>) -> Self {
        self.wrapping_add(&other.amount)
    }

    fn checked_offset(&self, other: AddressDiff<Self>) -> Option<Self> {
        self.checked_add(&other.amount)
    }
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
