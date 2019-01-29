extern crate num_traits;

use std::fmt::{Debug, Display};

use std::ops::{Add, Sub};

use num_traits::identities;
use num_traits::{Bounded, WrappingAdd, WrappingSub, CheckedAdd, CheckedSub};
                                                             // This is pretty wonk..

pub trait AddressDisplay {
    fn stringy(&self) -> String;
}

pub trait Address where Self:
    Debug + Display + AddressDisplay +
    Copy + Clone + Sized +
    Ord + Eq + PartialEq + Bounded +
    Add<Output=Self> + Sub<Output=Self> +
    WrappingAdd + WrappingSub +
    CheckedAdd + CheckedSub +
    identities::One + identities::Zero {
    fn to_linear(&self) -> usize;
}
/*
impl <T> Address for T where T: Sized + Ord + Add<Output=Self> + From<u16> + Into<usize> {
    fn to_linear(&self) -> usize { *self.into() }
}
*/

impl AddressDisplay for u32 {
    fn stringy(&self) -> String {
        format!("{:#x}", self)
    }
}

impl AddressDisplay for u16 {
    fn stringy(&self) -> String {
        format!("{:#x}", self)
    }
}

impl Address for u16 {
    fn to_linear(&self) -> usize { *self as usize }
}

impl Address for u32 {
    fn to_linear(&self) -> usize { *self as usize }
}

pub trait Decodable where Self: Sized {
    fn decode<'a, T: IntoIterator<Item=&'a u8>>(bytes: T) -> Option<Self>;
    fn decode_into<'a, T: IntoIterator<Item=&'a u8>>(&mut self, bytes: T) -> Option<()>;
}

pub trait Arch {
    type Address: Address + Debug;
    type Instruction: Decodable + LengthedInstruction<Unit=Self::Address>;
    type Operand;
}

pub trait LengthedInstruction {
    type Unit;
    fn len(&self) -> Self::Unit;
}
