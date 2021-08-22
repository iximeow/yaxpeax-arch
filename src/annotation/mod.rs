//! traits (and convenient impls) for decoders that also produce descriptions of parsed bit fields.
//!
//! the design of this API is discussed in [`yaxpeax-arch`
//! documentation](https://github.com/iximeow/yaxpeax-arch/blob/no-gods-no-/docs/0001-AnnotatingDecoder.md#descriptionsink).
//!
//! ## usage
//!
//! [`AnnotatingDecoder::decode_with_annotation`] decodes an instruction much like
//! [`crate::Decoder::decode_into`], but also reports descriptions of bit fields to a provided
//! [`DescriptionSink`]. [`VecSink`] is likely the `DescriptionSink` of interest to retain fields;
//! decoders are not required to make any guarantees about the order of descriptions, either by the
//! description's associated [`FieldDescription::id`], or with respect to the bits a
//! `FieldDescription` is reported against. fields may be described by multiple `FieldDescription`
//! with matching `id` and `desc` -- this is to describe data in an instruction where
//! non-contiguous bits are taken together for a single detail. for these cases, the various
//! `FieldDescription` must compare equal, and users of `yaxpeax-arch` can rely on this equivalence
//! for grouping bit ranges.
//!
//! in a generic setting, there isn't much to do with a `FieldDescription` other than display it. a
//! typical use might look something like:
//! ```
//! fn show_field_descriptions<A: Arch>(decoder: A::Decoder, buf: &[u8])
//! where
//!     A::Decoder: AnnotatingDecoder<A>,
//!     A::Instruction: fmt::Display, for<'data> U8Reader<'data>: Reader<A::Address, A::Word>,
//! {
//!     let mut inst = A::Instruction::default();
//!     let mut reader = U8Reader::new(buf);
//!     let mut sink: VecSink<<A::Decoder as AnnotatingDecoder<A>>::FieldDescription> = VecSink::new();
//!
//!     decoder.decode_with_annotation(&mut inst, &mut reader, &mut sink).unwrap();
//!
//!     println!("decoded instruction {}", inst);
//!     for (start, end, desc) in sink.records.iter() {
//!         println("  bits [{}, {}]: {}", start, end, desc);
//!     }
//! }
//! ```
//!
//! note that the range `[start, end]` for a reported span is _inclusive_. the `end`-th bit of a
//! an instruction's bit stream is described by the description.
//!
//! ## implementation guidance
//!
//! the typical implementation pattern is that an architecture's `Decoder` implements [`crate::Decoder`]
//! _and_ [`AnnotatingDecoder`], then callers are free to choose which style of decoding they want.
//! [`NullSink`] has a blanket impl of [`DescriptionSink`] for all possible descriptions, and
//! discards reported field descriptions. `decode_with_annotation` with annotations reported to a
//! `NullSink` must be functionally identical to a call to `Decoder::decode_into`.
//!
//! the important points:
//!
//! * `AnnotatingDecoder` is an **optional** implementation for decoders.
//! * `FieldDescription` in general is oriented towards human-directed output, but implementations
//! can be as precise as they want.
//! * since bit/byte order varies from architecture to architecture, a field's `start` and `end`
//! are defined with some ordering from the corresponding decoder crate. crates should describe the
//! bit ordering they select, and where possible, the bit ordering they describe should match
//! relevant ISA mauals.
//! * `FieldDescription` that return true for [`FieldDescription::is_separator`] are an exception
//! to bit span inclusivity: for these descriptions, the bit range should be `[b, b]` where `b` is
//! the last bit before the boundary being delimited. unlike other descriptions, `is_separator`
//! descriptions describe the space between bits `b` and `b+1`.
//! * if a description is to cover multiple bit fields, the reported `FieldDescription` must
//! be identical on `id` and `desc` for all involved bit fields.

use crate::{Arch, Reader};

use core::fmt::Display;

/// implementors of `DescriptionSink` receive descriptions of an instruction's disassembly process
/// and relevant offsets in the bitstream being decoded. descriptions are archtecture-specific, and
/// architectures are expected to be able to turn the bit-level `start` and `width` values into a
/// meaningful description of bits in the original instruction stream.
pub trait DescriptionSink<Descriptor> {
    /// inform this `DescriptionSink` of a `description` that was informed by bits `start` to
    /// `end` from the start of an instruction's decoding. `start` and `end` are only relative the
    /// instruction being decoded when this sink `DescriptionSink` provided, so they will have no
    /// relation to the position in an underlying data stream used for past or future instructions.
    fn record(&mut self, start: u32, end: u32, description: Descriptor);
}

pub struct NullSink;

impl<T> DescriptionSink<T> for NullSink {
    fn record(&mut self, _start: u32, _end: u32, _description: T) { }
}

#[cfg(feature = "std")]
pub struct VecSink<T: Clone + Display> {
    pub records: std::vec::Vec<(u32, u32, T)>
}

#[cfg(feature = "std")]
impl<T: Clone + Display> VecSink<T> {
    pub fn new() -> Self {
        VecSink { records: std::vec::Vec::new() }
    }
}

#[cfg(feature = "std")]
impl<T: Clone + Display> DescriptionSink<T> for VecSink<T> {
    fn record(&mut self, start: u32, end: u32, description: T) {
        self.records.push((start, end, description));
    }
}

pub trait FieldDescription {
    fn id(&self) -> u32;
    fn is_separator(&self) -> bool;
}

/// an interface to decode [`Arch::Instruction`] words from a reader of [`Arch::Word`]s, with the
/// decoder able to report descriptions of bits or fields in the instruction to a sink implementing
/// [`DescriptionSink`]. the sink may be [`NullSink`] to discard provided data. decoding with a
/// `NullSink` should behave identically to `Decoder::decode_into`. implementors are recommended to
/// implement `Decoder::decode_into` as a call to `AnnotatingDecoder::decode_with_annotation` if
/// implementing both traits.
pub trait AnnotatingDecoder<A: Arch + ?Sized> {
    type FieldDescription: FieldDescription + Clone + Display + PartialEq;

    fn decode_with_annotation<
        T: Reader<A::Address, A::Word>,
        S: DescriptionSink<Self::FieldDescription>
    >(&self, inst: &mut A::Instruction, words: &mut T, sink: &mut S) -> Result<(), A::DecodeError>;
}
