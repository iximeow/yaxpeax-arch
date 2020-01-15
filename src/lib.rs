#![no_std]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

use core::fmt::{self, Debug, Display};
use core::hash::Hash;

extern crate num_traits;
#[cfg(feature="use-serde")]
extern crate serde;
#[cfg(feature="colors")]
extern crate termion;

#[cfg(feature="use-serde")]
use serde::{Serialize, Deserialize};

mod address;
pub use address::{Address, AddressBase};

mod color;
pub use color::{Colorize, NoColors, YaxColors};

#[cfg(feature="colors")]
pub use color::ColorSettings;

pub mod display;

pub trait DecodeError {
    fn data_exhausted(&self) -> bool;
    fn bad_opcode(&self) -> bool;
    fn bad_operand(&self) -> bool;
}

pub trait Decoder<Inst> where Inst: Sized + Default {
    type Error: DecodeError + Debug + Display;

    fn decode<T: IntoIterator<Item=u8>>(&self, bytes: T) -> Result<Inst, Self::Error> {
        let mut inst = Inst::default();
        self.decode_into(&mut inst, bytes).map(|_: ()| inst)
    }

    fn decode_into<T: IntoIterator<Item=u8>>(&self, inst: &mut Inst, bytes: T) -> Result<(), Self::Error>;
}

#[cfg(feature="use-serde")]
pub trait Arch {
    type Address: Address + Debug + Hash + PartialEq + Eq + Serialize + for<'de> Deserialize<'de>;
    type Instruction: Instruction + LengthedInstruction<Unit=Self::Address> + Debug + Default;
    type DecodeError: DecodeError + Debug + Display;
    type Decoder: Decoder<Self::Instruction, Error=Self::DecodeError> + Default;
    type Operand;
}

#[cfg(not(feature="use-serde"))]
pub trait Arch {
    type Address: Address + Debug + Hash + PartialEq + Eq;
    type Instruction: Instruction + LengthedInstruction<Unit=Self::Address> + Debug + Default;
    type DecodeError: DecodeError + Debug + Display;
    type Decoder: Decoder<Self::Instruction, Error=Self::DecodeError> + Default;
    type Operand;
}

pub trait LengthedInstruction {
    type Unit;
    fn len(&self) -> Self::Unit;
    fn min_size() -> Self::Unit;
}

pub trait Instruction {
    fn well_defined(&self) -> bool;
}

pub trait ShowContextual<Addr, Ctx: ?Sized, Color: Display, T: fmt::Write, Y: YaxColors<Color>> {
    fn contextualize(&self, colors: &Y, address: Addr, context: Option<&Ctx>, out: &mut T) -> fmt::Result;
}

/*
impl <C: ?Sized, T: fmt::Write, U: Colorize<T>> ShowContextual<C, T> for U {
    fn contextualize(&self, colors: Option<&ColorSettings>, context: Option<&C>, out: &mut T) -> fmt::Result {
        self.colorize(colors, out)
    }
}
*/
