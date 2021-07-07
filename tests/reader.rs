use yaxpeax_arch::{Reader, U8Reader, U16le, U32le};

#[test]
fn reader_offset_is_words_not_bytes() {
    fn test_u16<T: Reader<u64, U16le>>(reader: &mut T) {
        reader.mark();
        assert_eq!(reader.offset(), 0);
        reader.next().unwrap();
        assert_eq!(reader.offset(), 1);
        reader.mark();
        reader.next().unwrap();
        assert_eq!(reader.offset(), 1);
        assert_eq!(reader.total_offset(), 2);
    }
    fn test_u32<T: Reader<u64, U32le>>(reader: &mut T) {
        reader.mark();
        assert_eq!(reader.offset(), 0);
        reader.next().unwrap();
        assert_eq!(reader.offset(), 1);
    }

    test_u16(&mut U8Reader::new(&[0x01, 0x02, 0x03, 0x04]));
    test_u32(&mut U8Reader::new(&[0x01, 0x02, 0x03, 0x04]));
}
