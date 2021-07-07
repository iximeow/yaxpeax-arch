use yaxpeax_arch::AddressBase;

mod reader;

#[test]
fn test_u16() {
    for l in 0..100 {
        for r in 0..=core::u16::MAX {
            assert_eq!(r.wrapping_offset(l.diff(&r).expect("u16 addresses always have valid diffs")), l);
        }
    }
}

#[test]
fn generic_error_can_bail() {
    use yaxpeax_arch::{Arch, Decoder, Reader};

    #[allow(dead_code)]
    fn decode<A: Arch, U: Into<impl Reader<A::Address, A::Word>>>(data: U, decoder: &A::Decoder) -> anyhow::Result<()> {
        let mut reader = data.into();
        decoder.decode(&mut reader)?;
        Ok(())
    }
}
#[test]
fn error_can_bail() {
    use yaxpeax_arch::{Arch, AddressDiff, Decoder, Reader, LengthedInstruction, Instruction, StandardDecodeError, U8Reader};
    struct TestIsa {}
    #[derive(Debug, Default)]
    struct TestInst {}
    impl Arch for TestIsa {
        type Word = u8;
        type Address = u64;
        type Instruction = TestInst;
        type Decoder = TestIsaDecoder;
        type DecodeError = StandardDecodeError;
        type Operand = ();
    }

    impl Instruction for TestInst {
        fn well_defined(&self) -> bool { true }
    }

    impl LengthedInstruction for TestInst {
        type Unit = AddressDiff<u64>;
        fn len(&self) -> Self::Unit { AddressDiff::from_const(1) }
        fn min_size() -> Self::Unit { AddressDiff::from_const(1) }
    }

    struct TestIsaDecoder {}

    impl Default for TestIsaDecoder {
        fn default() -> Self {
            TestIsaDecoder {}
        }
    }

    impl Decoder<TestIsa> for TestIsaDecoder {
        fn decode_into<T: Reader<u64, u8>>(&self, _inst: &mut TestInst, _words: &mut T) -> Result<(), StandardDecodeError> {

            Err(StandardDecodeError::ExhaustedInput)
        }
    }

    #[derive(Debug, PartialEq, thiserror::Error)]
    pub enum Error {
        #[error("decode error")]
        TestDecode(#[from] StandardDecodeError),
    }

    fn exercise_eq() -> Result<(), Error> {
        let mut reader = U8Reader::new(&[]);
        TestIsaDecoder::default().decode(&mut reader)?;
        Ok(())
    }

    assert_eq!(exercise_eq(), Err(Error::TestDecode(StandardDecodeError::ExhaustedInput)));
}
