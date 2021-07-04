#![no_std]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

use core::fmt::{self, Debug, Display};
use core::hash::Hash;

extern crate num_traits;
#[cfg(feature="use-serde")]
extern crate serde;
#[cfg(feature="use-serde")]
#[macro_use] extern crate serde_derive;
#[cfg(feature="colors")]
extern crate crossterm;

#[cfg(feature="use-serde")]
use serde::{Serialize, Deserialize};

mod address;
pub use address::{Address, AddressBase, AddressDiff, AddressDiffAmount, AddressDisplay};
pub use address::{AddressDisplayUsize, AddressDisplayU64, AddressDisplayU32, AddressDisplayU16};
#[cfg(feature="address-parse")]
pub use address::AddrParse;

mod color;
pub use color::{Colorize, NoColors, YaxColors};

#[cfg(feature="colors")]
pub use color::ColorSettings;

pub mod display;
mod reader;
pub use reader::{Reader, ReadError, U8Reader}; //, U16le, U16be, U32le, U32be, U64le, U64be};

pub trait DecodeError {
    fn data_exhausted(&self) -> bool;
    fn bad_opcode(&self) -> bool;
    fn bad_operand(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum StandardDecodeError {
    ExhaustedInput,
    InvalidOpcode,
    InvalidOperand,
}

impl fmt::Display for StandardDecodeError {
    fn fmt(&self, f:  &mut fmt::Formatter) -> fmt::Result {
        match self {
            StandardDecodeError::ExhaustedInput => write!(f, "exhausted input"),
            StandardDecodeError::InvalidOpcode => write!(f, "invalid opcode"),
            StandardDecodeError::InvalidOperand => write!(f, "invalid operand"),
        }
    }
}

impl DecodeError for StandardDecodeError {
    fn data_exhausted(&self) -> bool { *self == StandardDecodeError::ExhaustedInput }
    fn bad_opcode(&self) -> bool { *self == StandardDecodeError::InvalidOpcode }
    fn bad_operand(&self) -> bool { *self == StandardDecodeError::InvalidOperand }
}

pub trait Decoder<A: Arch + ?Sized> {
    fn decode<T: Reader<A::Address, A::Word>>(&self, words: &mut T) -> Result<A::Instruction, A::DecodeError> {
        let mut inst = A::Instruction::default();
        self.decode_into(&mut inst, words).map(|_: ()| inst)
    }

    fn decode_into<T: Reader<A::Address, A::Word>>(&self, inst: &mut A::Instruction, words: &mut T) -> Result<(), A::DecodeError>;
}

#[cfg(feature="use-serde")]
pub trait Arch {
    type Word: Debug + Display + PartialEq + Eq;
    type Address: Address + Debug + Hash + PartialEq + Eq + Serialize + for<'de> Deserialize<'de>;
    type Instruction: Instruction + LengthedInstruction<Unit=AddressDiff<Self::Address>> + Debug + Default + Sized;
    type DecodeError: DecodeError + Debug + Display;
    type Decoder: Decoder<Self> + Default;
    type Operand;
}

#[cfg(not(feature="use-serde"))]
pub trait Arch {
    type Word: Debug + Display + PartialEq + Eq;
    type Address: Address + Debug + Hash + PartialEq + Eq;
    type Instruction: Instruction + LengthedInstruction<Unit=AddressDiff<Self::Address>> + Debug + Default + Sized;
    type DecodeError: DecodeError + Debug + Display;
    type Decoder: Decoder<Self> + Default;
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

pub trait ShowContextual<Addr, Ctx: ?Sized, T: fmt::Write, Y: YaxColors> {
    fn contextualize(&self, colors: &Y, address: Addr, context: Option<&Ctx>, out: &mut T) -> fmt::Result;
}

/*
impl <C: ?Sized, T: fmt::Write, U: Colorize<T>> ShowContextual<C, T> for U {
    fn contextualize(&self, colors: Option<&ColorSettings>, context: Option<&C>, out: &mut T) -> fmt::Result {
        self.colorize(colors, out)
    }
}
*/
