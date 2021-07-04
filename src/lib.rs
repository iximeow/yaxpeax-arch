#![no_std]

use core::fmt::{self, Debug, Display};
use core::hash::Hash;

#[cfg(feature="use-serde")]
#[macro_use] extern crate serde_derive;

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
pub use reader::{Reader, ReadError, U8Reader, U16le, U16be, U32le, U32be, U64le, U64be};

/// the minimum set of errors a `yaxpeax-arch` disassembler may produce.
///
/// it is permissible for an implementor of `DecodeError` to have items that return `false` for
/// all these functions; decoders are permitted to error in way that `yaxpeax-arch` does not know
/// about.
pub trait DecodeError: PartialEq {
    /// did the decoder fail because it reached the end of input?
    fn data_exhausted(&self) -> bool;
    /// did the decoder error because the instruction's opcode is invalid?
    ///
    /// this may not be a sensical question for some instruction sets - `bad_opcode` should
    /// generally indicate an issue with the instruction itself. this is in contrast to one
    /// specific operand being invalid for the instruction, or some other issue to do with decoding
    /// data beyond the top-level instruction. the "opcode"/"operand" distinction is often fuzzy
    /// and left as best-effort for decoder implementors.
    fn bad_opcode(&self) -> bool;
    /// did the decoder error because an operand of the instruction to decode is invalid?
    ///
    /// similar to [`DecodeError::bad_opcode`], this is a subjective distinction and best-effort on
    /// the part of implementors.
    fn bad_operand(&self) -> bool;
    /// a human-friendly description of this decode error.
    fn description(&self) -> &'static str;
}

/// a minimal enum implementing `DecodeError`. this is intended to be enough for a low effort,
/// low-fidelity error taxonomy, without boilerplate of a `DecodeError` implementation.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum StandardDecodeError {
    ExhaustedInput,
    InvalidOpcode,
    InvalidOperand,
}

/// a slightly less minimal enum `DecodeError`. similar to `StandardDecodeError`, this is an
/// anti-boilerplate measure. it additionally provides `IncompleteDecoder`, making it suitable to
/// represent error kinds for decoders that are ... not yet complete.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum StandardPartialDecoderError {
    ExhaustedInput,
    InvalidOpcode,
    InvalidOperand,
    IncompleteDecoder,
}

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
impl std::error::Error for StandardDecodeError {
    fn description(&self) -> &str {
        <Self as DecodeError>::description(self)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for StandardPartialDecoderError {
    fn description(&self) -> &str {
        <Self as DecodeError>::description(self)
    }
}

impl fmt::Display for StandardDecodeError {
    fn fmt(&self, f:  &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl fmt::Display for StandardPartialDecoderError {
    fn fmt(&self, f:  &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl DecodeError for StandardDecodeError {
    fn data_exhausted(&self) -> bool { *self == StandardDecodeError::ExhaustedInput }
    fn bad_opcode(&self) -> bool { *self == StandardDecodeError::InvalidOpcode }
    fn bad_operand(&self) -> bool { *self == StandardDecodeError::InvalidOperand }
    fn description(&self) -> &'static str {
        match self {
            StandardDecodeError::ExhaustedInput => "exhausted input",
            StandardDecodeError::InvalidOpcode => "invalid opcode",
            StandardDecodeError::InvalidOperand => "invalid operand",
        }
    }
}

impl DecodeError for StandardPartialDecoderError {
    fn data_exhausted(&self) -> bool { *self == StandardPartialDecoderError::ExhaustedInput }
    fn bad_opcode(&self) -> bool { *self == StandardPartialDecoderError::InvalidOpcode }
    fn bad_operand(&self) -> bool { *self == StandardPartialDecoderError::InvalidOperand }
    fn description(&self) -> &'static str {
        match self {
            StandardPartialDecoderError::ExhaustedInput => "exhausted input",
            StandardPartialDecoderError::InvalidOpcode => "invalid opcode",
            StandardPartialDecoderError::InvalidOperand => "invalid operand",
            StandardPartialDecoderError::IncompleteDecoder => "incomplete decoder",
        }
    }
}

/// an interface to decode [`Arch::Instruction`] words from a reader of [`Arch::Word`]s. errors are
/// the architecture-defined [`DecodeError`] implemention.
pub trait Decoder<A: Arch + ?Sized> {
    /// decode one instruction for this architecture from the [`yaxpeax_arch::Reader`] of this
    /// architecture's `Word`.
    fn decode<T: Reader<A::Address, A::Word>>(&self, words: &mut T) -> Result<A::Instruction, A::DecodeError> {
        let mut inst = A::Instruction::default();
        self.decode_into(&mut inst, words).map(|_: ()| inst)
    }

    /// decode one instruction for this architecture from the [`yaxpeax_arch::Reader`] of this
    /// architecture's `Word`, writing into the provided `inst`.
    ///
    /// SAFETY:
    ///
    /// while `inst` MUST be left in a state that does not violate Rust's safety guarantees,
    /// implementors are NOT obligated to leave `inst` in a semantically meaningful state if
    /// decoding fails. if `decode_into` returns an error, callers may find contradictory and
    /// useless information in `inst`, as well as *stale data* from whatever was passed in.
    fn decode_into<T: Reader<A::Address, A::Word>>(&self, inst: &mut A::Instruction, words: &mut T) -> Result<(), A::DecodeError>;
}

#[cfg(feature = "use-serde")]
pub trait AddressBounds: Address + Debug + Hash + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> {}
#[cfg(not(feature = "use-serde"))]
pub trait AddressBounds: Address + Debug + Hash + PartialEq + Eq {}

#[cfg(feature = "use-serde")]
impl<T> AddressBounds for T where T: Address + Debug + Hash + PartialEq + Eq + Serialize + for<'de> Deserialize<'de> {}
#[cfg(not(feature = "use-serde"))]
impl<T> AddressBounds for T where T: Address + Debug + Hash + PartialEq + Eq {}

#[cfg(feature = "std")]
trait DecodeErrorBounds: DecodeError + std::error::Error + Debug + Display {}
#[cfg(not(feature = "std"))]
trait DecodeErrorBounds: DecodeError + Debug + Display {}

/// a collection of associated type parameters that constitute the definitions for an instruction
/// set. `Arch` provides an `Instruction` and its associated `Operand`s, which is guaranteed to be
/// decodable by this `Arch::Decoder`. `Arch::Decoder` can always be constructed with a `Default`
/// implementation, and decodes from a `Reader<Arch::Address, Arch::Word>`.
///
/// `Arch` is suitable as the foundational trait to implement more complex logic on top of; for
/// example, it would be entirely expected to have a
/// ```text
/// pub fn emulate<A: Arch, E: Emulator<A>>(
///     reader: &mut Reader<A::Address, A::Word>,
///     emu: &mut E
/// ) -> Result<A::Address, DecodeOrEvaluationError>;
/// ```
///
/// in some library built on top of `yaxpeax-arch`.
pub trait Arch {
    type Word: Debug + Display + PartialEq + Eq;
    type Address: AddressBounds;
    type Instruction: Instruction + LengthedInstruction<Unit=AddressDiff<Self::Address>> + Debug + Default + Sized;
    type DecodeError: DecodeError + Debug + Display;
    type Decoder: Decoder<Self> + Default;
    type Operand;
}

/// instructions have lengths, and minimum possible sizes for advancing a decoder on error.
///
/// unfortunately, this means calling `x.len()` for some `Arch::Instruction` requires importing
/// this trait. sorry.
pub trait LengthedInstruction {
    type Unit;
    /// the length, in terms of `Unit`, of this instruction. because `Unit` will be a diff of an
    /// architecture's `Address` type, this almost always is a number of bytes. implementations
    /// should indicate if this is ever not the case.
    fn len(&self) -> Self::Unit;
    /// the length, in terms of `Unit`, of the shortest possible instruction in a given
    /// architecture.. because `Unit` will be a diff of an architecture's `Address` type, this
    /// almost always is a number of bytes. implementations should indicate if this is ever not the
    /// case.
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
