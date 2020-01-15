use crate::YaxColors;

use core::fmt;
use core::num::Wrapping;
use core::ops::Neg;

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

pub fn format_number_i32<W: fmt::Write, Color: fmt::Display, Y: YaxColors<Color>>(colors: &Y, f: &mut W, i: i32, hint: NumberStyleHint) -> fmt::Result {
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

pub fn u8_hex(value: u8) -> SignedHexDisplay<i8> {
    SignedHexDisplay {
        value: value as i8,
        negative: false,
    }
}

pub fn signed_i8_hex(imm: i8) -> SignedHexDisplay<i8> {
    SignedHexDisplay {
        value: imm,
        negative: imm < 0,
    }
}

pub fn u16_hex(value: u16) -> SignedHexDisplay<i16> {
    SignedHexDisplay {
        value: value as i16,
        negative: false,
    }
}

pub fn signed_i16_hex(imm: i16) -> SignedHexDisplay<i16> {
    SignedHexDisplay {
        value: imm,
        negative: imm < 0,
    }
}

pub fn u32_hex(value: u32) -> SignedHexDisplay<i32> {
    SignedHexDisplay {
        value: value as i32,
        negative: false,
    }
}

pub fn signed_i32_hex(imm: i32) -> SignedHexDisplay<i32> {
    SignedHexDisplay {
        value: imm,
        negative: imm < 0,
    }
}

pub fn u64_hex(value: u64) -> SignedHexDisplay<i64> {
    SignedHexDisplay {
        value: value as i64,
        negative: false,
    }
}

pub fn signed_i64_hex(imm: i64) -> SignedHexDisplay<i64> {
    SignedHexDisplay {
        value: imm,
        negative: imm < 0,
    }
}
