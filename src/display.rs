// allow use of deprecated items in this module since some functions using `SignedHexDisplay` still
// exist here
#![allow(deprecated)]

use crate::YaxColors;

use core::fmt;
use core::num::Wrapping;
use core::ops::Neg;

mod display_sink;

pub use display_sink::{DisplaySink, FmtSink};
#[cfg(feature = "alloc")]
pub use display_sink::InstructionTextSink;

/// translate a byte in range `[0, 15]` to a lowercase base-16 digit.
///
/// if `c` is in range, the output is always valid as the sole byte in a utf-8 string. if `c` is out
/// of range, the returned character might not be a valid single-byte utf-8 codepoint.
#[cfg(feature = "alloc")] // this function is of course not directly related to alloc, but it's only needed by impls that themselves are only present with alloc.
fn u8_to_hex(c: u8) -> u8 {
    // this conditional branch is faster than a lookup for... most architectures (especially x86
    // with cmov)
    if c < 10 {
        b'0' + c
    } else {
        b'a' + c - 10
    }
}

#[deprecated(since="0.3.0", note="format_number_i32 does not optimize as expected and will be removed in the future. see DisplaySink instead.")]
pub enum NumberStyleHint {
    Signed,
    HexSigned,
    SignedWithSign,
    HexSignedWithSign,
    SignedWithSignSplit,
    HexSignedWithSignSplit,
    Unsigned,
    HexUnsigned,
    UnsignedWithSign,
    HexUnsignedWithSign
}

#[deprecated(since="0.3.0", note="format_number_i32 is both slow and incorrect: YaxColors may not result in correct styling when writing anywhere other than a terminal, and both stylin and formatting does not inline as well as initially expected. see DisplaySink instead.")]
pub fn format_number_i32<W: fmt::Write, Y: YaxColors>(colors: &Y, f: &mut W, i: i32, hint: NumberStyleHint) -> fmt::Result {
    match hint {
        NumberStyleHint::Signed => {
            write!(f, "{}", colors.number(i))
        },
        NumberStyleHint::HexSigned => {
            write!(f, "{}", colors.number(signed_i32_hex(i)))
        },
        NumberStyleHint::Unsigned => {
            write!(f, "{}", colors.number(i as u32))
        },
        NumberStyleHint::HexUnsigned => {
            write!(f, "{}", colors.number(u32_hex(i as u32)))
        },
        NumberStyleHint::SignedWithSignSplit => {
            if i == core::i32::MIN {
                write!(f, "- {}", colors.number("2147483647"))
            } else if i < 0 {
                write!(f, "- {}", colors.number(-Wrapping(i)))
            } else {
                write!(f, "+ {}", colors.number(i))
            }
        }
        NumberStyleHint::HexSignedWithSignSplit => {
            if i == core::i32::MIN {
                write!(f, "- {}", colors.number("0x7fffffff"))
            } else if i < 0 {
                write!(f, "- {}", colors.number(u32_hex((-Wrapping(i)).0 as u32)))
            } else {
                write!(f, "+ {}", colors.number(u32_hex(i as u32)))
            }
        },
        NumberStyleHint::HexSignedWithSign => {
            write!(f, "{}", signed_i32_hex(i))
        },
        NumberStyleHint::SignedWithSign => {
            write!(f, "{:+}", i)
        }
        NumberStyleHint::HexUnsignedWithSign => {
            write!(f, "{:+#x}", i as u32)
        },
        NumberStyleHint::UnsignedWithSign => {
            write!(f, "{:+}", i as u32)
        }
    }
}

#[deprecated(since="0.3.0", note="SignedHexDisplay does not optimize like expected and will be removed in the future. see DisplaySink instead.")]
pub struct SignedHexDisplay<T: core::fmt::LowerHex + Neg> {
    value: T,
    negative: bool
}

impl<T: fmt::LowerHex + Neg + Copy> fmt::Display for SignedHexDisplay<T> where Wrapping<T>: Neg, <Wrapping<T> as Neg>::Output: fmt::LowerHex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.negative {
            write!(f, "-{:#x}", -Wrapping(self.value))
        } else {
            write!(f, "{:#x}", self.value)
        }
    }
}

#[deprecated(since="0.3.0", note="u8_hex does not optimize like expected and will be removed in the future. see DisplaySink instead.")]
pub fn u8_hex(value: u8) -> SignedHexDisplay<i8> {
    SignedHexDisplay {
        value: value as i8,
        negative: false,
    }
}

#[deprecated(since="0.3.0", note="signed_i8_hex does not optimize like expected and will be removed in the future. see DisplaySink instead.")]
pub fn signed_i8_hex(imm: i8) -> SignedHexDisplay<i8> {
    SignedHexDisplay {
        value: imm,
        negative: imm < 0,
    }
}

#[deprecated(since="0.3.0", note="u16_hex does not optimize like expected and will be removed in the future. see DisplaySink instead.")]
pub fn u16_hex(value: u16) -> SignedHexDisplay<i16> {
    SignedHexDisplay {
        value: value as i16,
        negative: false,
    }
}

#[deprecated(since="0.3.0", note="signed_i16_hex does not optimize like expected and will be removed in the future. see DisplaySink instead.")]
pub fn signed_i16_hex(imm: i16) -> SignedHexDisplay<i16> {
    SignedHexDisplay {
        value: imm,
        negative: imm < 0,
    }
}

#[deprecated(since="0.3.0", note="u32_hex does not optimize like expected and will be removed in the future. see DisplaySink instead.")]
pub fn u32_hex(value: u32) -> SignedHexDisplay<i32> {
    SignedHexDisplay {
        value: value as i32,
        negative: false,
    }
}

#[deprecated(since="0.3.0", note="signed_i32_hex does not optimize like expected and will be removed in the future. see DisplaySink instead.")]
pub fn signed_i32_hex(imm: i32) -> SignedHexDisplay<i32> {
    SignedHexDisplay {
        value: imm,
        negative: imm < 0,
    }
}

#[deprecated(since="0.3.0", note="u64_hex does not optimize like expected and will be removed in the future. see DisplaySink instead.")]
pub fn u64_hex(value: u64) -> SignedHexDisplay<i64> {
    SignedHexDisplay {
        value: value as i64,
        negative: false,
    }
}

#[deprecated(since="0.3.0", note="signed_i64_hex does not optimize like expected and will be removed in the future. see DisplaySink instead.")]
pub fn signed_i64_hex(imm: i64) -> SignedHexDisplay<i64> {
    SignedHexDisplay {
        value: imm,
        negative: imm < 0,
    }
}
