//! tools to test the correctness of `yaxpeax-arch` trait implementations.

use core::fmt;
use core::fmt::Write;

use crate::display::DisplaySink;

/// `DisplaySinkValidator` is a `DisplaySink` that panics if invariants required of
/// `DisplaySink`-writing functions are not upheld.
///
/// there are two categories of invariants that `DisplaySinkValidator` validates.
///
/// first, this panics if spans are not `span_end_*`-ed in first-in-last-out order with
/// corresponding `span_start_*. second, this panics if `write_lt_*` functions are ever provided
/// inputs longer than the corresponding maximum length.
///
/// functions that write to a `DisplaySink` are strongly encouraged to come with fuzzing that for
/// all inputs `DisplaySinkValidator` does not panic.
pub struct DisplaySinkValidator {
    spans: alloc::vec::Vec<&'static str>,
}

impl DisplaySinkValidator {
    pub fn new() -> Self {
        Self { spans: alloc::vec::Vec::new() }
    }
}

impl core::ops::Drop for DisplaySinkValidator {
    fn drop(&mut self) {
        if self.spans.len() != 0 {
            panic!("DisplaySinkValidator dropped with open spans");
        }
    }
}

impl fmt::Write for DisplaySinkValidator {
    fn write_str(&mut self, _s: &str) -> Result<(), fmt::Error> {
        Ok(())
    }
    fn write_char(&mut self, _c: char) -> Result<(), fmt::Error> {
        Ok(())
    }
}

impl DisplaySink for DisplaySinkValidator {
    unsafe fn write_lt_32(&mut self, s: &str) -> Result<(), fmt::Error> {
        if s.len() >= 32 {
            panic!("DisplaySinkValidator::write_lt_32 was given a string longer than the maximum permitted length");
        }

        self.write_str(s)
    }
    unsafe fn write_lt_16(&mut self, s: &str) -> Result<(), fmt::Error> {
        if s.len() >= 16 {
            panic!("DisplaySinkValidator::write_lt_16 was given a string longer than the maximum permitted length");
        }

        self.write_str(s)
    }
    unsafe fn write_lt_8(&mut self, s: &str) -> Result<(), fmt::Error> {
        if s.len() >= 8 {
            panic!("DisplaySinkValidator::write_lt_8 was given a string longer than the maximum permitted length");
        }

        self.write_str(s)
    }

    fn span_start_immediate(&mut self) {
        self.spans.push("immediate");
    }

    fn span_end_immediate(&mut self) {
        let last = self.spans.pop().expect("item to pop");
        assert_eq!(last, "immediate");
    }

    fn span_start_register(&mut self) {
        self.spans.push("register");
    }

    fn span_end_register(&mut self) {
        let last = self.spans.pop().expect("item to pop");
        assert_eq!(last, "register");
    }

    fn span_start_opcode(&mut self) {
        self.spans.push("opcode");
    }

    fn span_end_opcode(&mut self) {
        let last = self.spans.pop().expect("item to pop");
        assert_eq!(last, "opcode");
    }

    fn span_start_program_counter(&mut self) {
        self.spans.push("program counter");
    }

    fn span_end_program_counter(&mut self) {
        let last = self.spans.pop().expect("item to pop");
        assert_eq!(last, "program counter");
    }

    fn span_start_number(&mut self) {
        self.spans.push("number");
    }

    fn span_end_number(&mut self) {
        let last = self.spans.pop().expect("item to pop");
        assert_eq!(last, "number");
    }

    fn span_start_address(&mut self) {
        self.spans.push("address");
    }

    fn span_end_address(&mut self) {
        let last = self.spans.pop().expect("item to pop");
        assert_eq!(last, "address");
    }

    fn span_start_function_expr(&mut self) {
        self.spans.push("function expr");
    }

    fn span_end_function_expr(&mut self) {
        let last = self.spans.pop().expect("item to pop");
        assert_eq!(last, "function expr");
    }
}

/// `DisplaySinkWriteComparator` helps test that two `DisplaySink` implementations which should
/// produce the same output actually do.
///
/// this is most useful for cases like testing specialized `write_lt_*` functions, which ought to
/// behave the same as if `write_str()` were called instead and so can be used as a very simple
/// oracle.
///
/// this is somewhat less useful when the sinks are expected to produce unequal text, such as when
/// one sink writes ANSI color sequences and the other does not.
pub struct DisplaySinkWriteComparator<'sinks, T: DisplaySink, U: DisplaySink> {
    sink1: &'sinks mut T,
    sink1_check: fn(&T) -> &str,
    sink2: &'sinks mut U,
    sink2_check: fn(&U) -> &str,
}

impl<'sinks, T: DisplaySink, U: DisplaySink> DisplaySinkWriteComparator<'sinks, T, U> {
    pub fn new(
        t: &'sinks mut T, t_check: fn(&T) -> &str,
        u: &'sinks mut U, u_check: fn(&U) -> &str
    ) -> Self {
        Self {
            sink1: t,
            sink1_check: t_check,
            sink2: u,
            sink2_check: u_check,
        }
    }

    fn compare_sinks(&self) {
        let sink1_text = (self.sink1_check)(self.sink1);
        let sink2_text = (self.sink2_check)(self.sink2);

        if sink1_text != sink2_text {
            panic!("sinks produced different output: {} != {}", sink1_text, sink2_text);
        }
    }
}

impl<'sinks, T: DisplaySink, U: DisplaySink> DisplaySink for DisplaySinkWriteComparator<'sinks, T, U> {
    fn write_u8(&mut self, v: u8) -> Result<(), fmt::Error> {
        self.sink1.write_u8(v).expect("write to sink1 succeeds");
        self.sink2.write_u8(v).expect("write to sink2 succeeds");
        self.compare_sinks();
        Ok(())
    }
}

impl<'sinks, T: DisplaySink, U: DisplaySink> fmt::Write for DisplaySinkWriteComparator<'sinks, T, U> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.sink1.write_str(s).expect("write to sink1 succeeds");
        self.sink2.write_str(s).expect("write to sink2 succeeds");
        Ok(())
    }
    fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
        self.sink1.write_char(c).expect("write to sink1 succeeds");
        self.sink2.write_char(c).expect("write to sink2 succeeds");
        Ok(())
    }
}
