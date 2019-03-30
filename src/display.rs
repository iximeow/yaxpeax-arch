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

pub fn format_number_i32(i: i32, hint: NumberStyleHint) -> String {
    match hint {
        NumberStyleHint::Signed => {
            format!("{}", i)
        },
        NumberStyleHint::HexSigned => {
            if i == std::i32::MIN {
                format!("-0x7fffffff")
            } else if i < 0 {
                format!("-{:#x}", -i)
            } else {
                format!("{:#x}", i)
            }
        },
        NumberStyleHint::Unsigned => {
            format!("{}", i as u32)
        },
        NumberStyleHint::HexUnsigned => {
            format!("{:#x}", i as u32)
        },
        NumberStyleHint::SignedWithSignSplit => {
            if i == std::i32::MIN {
                format!("- 2147483647")
            } else if i < 0 {
                format!("- {}", -i)
            } else {
                format!("+ {}", i)
            }
        }
        NumberStyleHint::HexSignedWithSignSplit => {
            if i == std::i32::MIN {
                format!("- 0x7fffffff")
            } else if i < 0 {
                format!("- {:#x}", -i)
            } else {
                format!("+ {:#x}", i)
            }
        },
        NumberStyleHint::HexSignedWithSign => {
            if i == std::i32::MIN {
                format!("-0x7fffffff")
            } else if i < 0 {
                format!("-{:#x}", -i)
            } else {
                format!("+{:#x}", i)
            }
        },
        NumberStyleHint::SignedWithSign => {
            format!("{:+}", i)
        }
        NumberStyleHint::HexUnsignedWithSign => {
            format!("{:+#x}", i as u32)
        },
        NumberStyleHint::UnsignedWithSign => {
            format!("{:+}", i as u32)
        }
    }
}

pub fn signed_i8_hex(imm: i8) -> String {
    let (sign, imm) = if imm == std::i8::MIN {
        (false, imm)
    } else if imm < 0 {
        (true, -imm)
    } else {
        (false, imm)
    };
    format!("{}{:#x}", sign, imm)
}

pub fn signed_i16_hex(imm: i16) -> String {
    let (sign, imm) = if imm == std::i16::MIN {
        (false, imm)
    } else if imm < 0 {
        (true, -imm)
    } else {
        (false, imm)
    };
    format!("{}{:#x}", sign, imm)
}

pub fn signed_i32_hex(imm: i32) -> String {
    let (sign, imm) = if imm == std::i32::MIN {
        (false, imm)
    } else if imm < 0 {
        (true, -imm)
    } else {
        (false, imm)
    };
    format!("{}{:#x}", sign, imm)
}

pub fn signed_i64_hex(imm: i64) -> String {
    let (sign, imm) = if imm == std::i64::MIN {
        (false, imm)
    } else if imm < 0 {
        (true, -imm)
    } else {
        (false, imm)
    };
    format!("{}{:#x}", sign, imm)
}
