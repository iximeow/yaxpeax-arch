#![no_std]

use yaxpeax_arch::AddressBase;

#[test]
fn test_u16() {
    for l in 0..100 {
        for r in 0..=core::u16::MAX {
            assert_eq!(r.wrapping_offset(l.diff(&r).expect("u16 addresses always have valid diffs")), l);
        }
    }
}
