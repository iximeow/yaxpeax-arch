/*
#[test]
fn sinks_are_equivalent() {
    use yaxpeax_arch::display::NoColorsSink;
    use yaxpeax_arch::testkit::DisplaySinkWriteComparator;

    let mut bare = String::new();
    let mut through_sink = String::new();
    for i in 0..u64::MAX {
        bare.clear();
        through_sink.clear();
        let mut out = NoColorsSink {
            out: &mut through_sink
        };
        let mut comparator = DisplaySinkWriteComparator {
            sink1: &mut out,
            sink1_check: |sink| { sink.out.as_str() },
            sink2: &mut bare,
            sink2_check: |sink| { sink.as_str() },
        };
    }
}
*/

#[test]
#[allow(deprecated)]
fn formatters_are_not_feature_gated() {
    use yaxpeax_arch::display::{
        u8_hex, u16_hex, u32_hex, u64_hex,
        signed_i8_hex, signed_i16_hex, signed_i32_hex, signed_i64_hex
    };
    let _ = u8_hex(10);
    let _ = u16_hex(10);
    let _ = u32_hex(10);
    let _ = u64_hex(10);
    let _ = signed_i8_hex(10);
    let _ = signed_i16_hex(10);
    let _ = signed_i32_hex(10);
    let _ = signed_i64_hex(10);
}

#[cfg(feature="alloc")]
#[test]
fn display_sink_write_hex_helpers() {
    use yaxpeax_arch::display::{DisplaySink};

    // for u8/i8/u16/i16 we can exhaustively test. we'll leave the rest for fuzzers.
    let mut buf = String::new();
    for i in 0..=u8::MAX {
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
    }

    for i in 0..=u16::MAX {
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
}
