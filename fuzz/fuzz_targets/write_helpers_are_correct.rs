#![no_main]
use libfuzzer_sys::fuzz_target;
use yaxpeax_arch::display::DisplaySink;

use std::convert::TryInto;

fuzz_target!(|data: &[u8]| {
    let mut buf = String::new();
    match data.len() {
        1 => {
            let i = data[0];

            buf.clear();
            buf.write_u8(i).expect("write succeeds");
            assert_eq!(buf, format!("{:x}", i));

            buf.clear();
            buf.write_prefixed_u8(i).expect("write succeeds");
            assert_eq!(buf, format!("0x{:x}", i));

            let expected = if (i as i8) < 0 {
                format!("-0x{:x}", (i as i8).unsigned_abs())
            } else {
                format!("0x{:x}", i)
            };

            buf.clear();
            buf.write_prefixed_i8(i as i8).expect("write succeeds");
            assert_eq!(buf, expected);
        },
        2 => {
            let i: u16 = u16::from_le_bytes(data.try_into().expect("checked the size is right"));

            buf.clear();
            buf.write_u16(i).expect("write succeeds");
            assert_eq!(buf, format!("{:x}", i));

            buf.clear();
            buf.write_prefixed_u16(i).expect("write succeeds");
            assert_eq!(buf, format!("0x{:x}", i));

            let expected = if (i as i16) < 0 {
                format!("-0x{:x}", (i as i16).unsigned_abs())
            } else {
                format!("0x{:x}", i)
            };

            buf.clear();
            buf.write_prefixed_i16(i as i16).expect("write succeeds");
            assert_eq!(buf, expected);
        }
        4 => {
            let i: u32 = u32::from_le_bytes(data.try_into().expect("checked the size is right"));

            buf.clear();
            buf.write_u32(i).expect("write succeeds");
            assert_eq!(buf, format!("{:x}", i));

            buf.clear();
            buf.write_prefixed_u32(i).expect("write succeeds");
            assert_eq!(buf, format!("0x{:x}", i));

            let expected = if (i as i32) < 0 {
                format!("-0x{:x}", (i as i32).unsigned_abs())
            } else {
                format!("0x{:x}", i)
            };

            buf.clear();
            buf.write_prefixed_i32(i as i32).expect("write succeeds");
            assert_eq!(buf, expected);
        },
        8 => {
            let i: u64 = u64::from_le_bytes(data.try_into().expect("checked the size is right"));

            buf.clear();
            buf.write_u64(i).expect("write succeeds");
            assert_eq!(buf, format!("{:x}", i));

            buf.clear();
            buf.write_prefixed_u64(i).expect("write succeeds");
            assert_eq!(buf, format!("0x{:x}", i));

            let expected = if (i as i64) < 0 {
                format!("-0x{:x}", (i as i64).unsigned_abs())
            } else {
                format!("0x{:x}", i)
            };

            buf.clear();
            buf.write_prefixed_i64(i as i64).expect("write succeeds");
            assert_eq!(buf, expected);
        },
        _ => {}
    }
});
