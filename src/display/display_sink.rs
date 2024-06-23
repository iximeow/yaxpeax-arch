use core::fmt;

/// `DisplaySink` allows client code to collect output and minimal markup. this is currently used
/// in formatting instructions for two reasons:
/// * `DisplaySink` implementations have the opportunity to collect starts and ends of tokens at
///   the same time as collecting output itself.
/// * `DisplaySink` implementations provide specialized functions for writing strings in
///   circumstances where a simple "use `core::fmt`" might incur unwanted overhead.
///
/// ## spans
///
/// spans are out-of-band indicators for the meaning of data written to this sink. when a
/// `span_start_<foo>` function is called, data written until a matching `span_end_<foo>` can be
/// considered the text corresponding to `<foo>`.
///
/// spans are entered and exited in a FILO manner. implementations of `DisplaySink` are explicitly
/// allowed to depend on this fact. functions writing to a `DisplaySink` must exit spans in reverse
/// order to when they are entered. a function that has a call sequence like
/// ```text
/// sink.span_start_operand();
/// sink.span_start_immediate();
/// sink.span_end_operand();
/// ```
/// is in error.
///
/// spans are reported through the `span_start_*` and `span_end_*` families of functions to avoid
/// constraining implementations into tracking current output offset (which may not be knowable) or
/// span size (which may be knowable, but incur additional overhead to compute or track). if the
/// task for a span is to simply emit VT100 color codes, for example, implementations avoid the
/// overhead of tracking offsets.
///
/// default implementations of the `span_start_*` and `span_end_*` functions are to do nothing. a
/// no-op `span_start_*` or `span_end_*` allows rustc to elimiate such calls at compile time for
/// `DisplaySink` that are uninterested in the corresponding span type.
///
/// # write helpers (`write_*`)
///
/// the `write_*` helpers on `DisplaySink` may be able to take advantage of contraints described in
/// documentation here to better support writing some kinds of inputs than a fully-general solution
/// (such as `core::fmt`) might be able to yield.
///
/// currently there are two motivating factors for `write_*` helpers:
///
/// instruction formatting often involves writing small but variable-size strings, such as register
/// names, which is something of a pathological case for string appending as Rust currently exists:
/// this often becomes `memcpy` and specifically a call to the platform's `memcpy` (rather than an
/// inlined `rep movsb`) just to move 3-5 bytes. one relevant Rust issue for reference:
/// <https://github.com/rust-lang/rust/issues/92993#issuecomment-2028915232>
///
/// there are similar papercuts around formatting integers as base-16 numbers, such as
/// <https://github.com/rust-lang/rust/pull/122770>. in isolation and in most applications these are
/// not a significant source of overhead. but for programs bounded on decoding and printing
/// instructions, these can add up to significant overhead - on the order of 10-20% of total
/// runtime.
///
/// ## example
///
/// a simple call sequence to `DisplaySink` might look something like:
/// ```compile_fail
/// sink.span_start_operand()
/// sink.write_char('[')
/// sink.span_start_register()
/// sink.write_fixed_size("rbp")
/// sink.span_end_register()
/// sink.write_char(']')
/// sink.span_end_operand()
/// ```
/// which writes the text `[rbp]`, telling sinks that the operand begins at `[`, ends after `]`,
/// and `rbp` is a register in that operand.
///
/// ## extensibility
///
/// additional `span_{start,end}_*` helpers may be added over time - in the above example, one
/// future addition might be to add a new `effective_address` span that is started before
/// `register` and ended after `register. for an operand like `\[rbp\]` the effective address span
/// would exactly match a corresponding register span, but in more complicated scenarios like
/// `[rsp + rdi * 4 + 0x50]` the effective address would be all of `rsp + rdi * 4 + 0x50`.
///
/// additional spans are expected to be added as needed. it is not immediately clear how to add
/// support for more architecture-specific concepts (such as itanium predicate registers) would be
/// supported yet, and so architecture-specific concepts may be expressed on `DisplaySink` if the
/// need arises.
///
/// new `span_{start,end}_*` helpers will be defaulted as no-op. additions to this trait will be
/// minor version bumps, so users should take care to not add custom functions starting with
/// `span_start_` or `span_end_` to structs implementing `DisplaySink`.
pub trait DisplaySink: fmt::Write {
    #[inline(always)]
    fn write_fixed_size(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.write_str(s)
    }

    /// write a string to this sink that is less than 32 bytes. this is provided for optimization
    /// opportunities when writing a variable-length string with known max size.
    ///
    /// SAFETY: the provided `s` must be less than 32 bytes. if the provided string is longer than
    /// 31 bytes, implementations may only copy part of a multi-byte codepoint while writing to a
    /// utf-8 string. this may corrupt Rust strings.
    unsafe fn write_lt_32(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.write_str(s)
    }
    /// write a string to this sink that is less than 16 bytes. this is provided for optimization
    /// opportunities when writing a variable-length string with known max size.
    ///
    /// SAFETY: the provided `s` must be less than 16 bytes. if the provided string is longer than
    /// 15 bytes, implementations may only copy part of a multi-byte codepoint while writing to a
    /// utf-8 string. this may corrupt Rust strings.
    unsafe fn write_lt_16(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.write_str(s)
    }
    /// write a string to this sink that is less than 8 bytes. this is provided for optimization
    /// opportunities when writing a variable-length string with known max size.
    ///
    /// SAFETY: the provided `s` must be less than 8 bytes. if the provided string is longer than
    /// 7 bytes, implementations may only copy part of a multi-byte codepoint while writing to a
    /// utf-8 string. this may corrupt Rust strings.
    unsafe fn write_lt_8(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.write_str(s)
    }

    /// write a u8 to the output as a base-16 integer.
    ///
    /// this corresponds to the Rust format specifier `{:x}` - see [`std::fmt::LowerHex`] for more.
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    fn write_u8(&mut self, v: u8) -> Result<(), core::fmt::Error> {
        write!(self, "{:x}", v)
    }
    /// write a u8 to the output as a base-16 integer with leading `0x`.
    ///
    /// this corresponds to the Rust format specifier `{#:x}` - see [`std::fmt::LowerHex`] for more.
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    fn write_prefixed_u8(&mut self, v: u8) -> Result<(), core::fmt::Error> {
        self.write_fixed_size("0x")?;
        self.write_u8(v)
    }
    /// write an i8 to the output as a base-16 integer with leading `0x`, and leading `-` if the
    /// value is negative.
    ///
    /// there is no matching `std` formatter, so some examples here:
    /// ```text
    /// sink.write_prefixed_i8(-0x60); // writes `-0x60` to the sink
    /// sink.write_prefixed_i8(127); // writes `0x7f` to the sink
    /// sink.write_prefixed_i8(-128); // writes `-0x80` to the sink
    /// ```
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    fn write_prefixed_i8(&mut self, v: i8) -> Result<(), core::fmt::Error> {
        let v = if v < 0 {
            self.write_char('-')?;
            v.unsigned_abs()
        } else {
            v as u8
        };
        self.write_prefixed_u8(v)
    }
    /// write a u16 to the output as a base-16 integer.
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    fn write_u16(&mut self, v: u16) -> Result<(), core::fmt::Error> {
        write!(self, "{:x}", v)
    }
    /// write a u16 to the output as a base-16 integer with leading `0x`.
    ///
    /// this corresponds to the Rust format specifier `{#:x}` - see [`std::fmt::LowerHex`] for more.
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    fn write_prefixed_u16(&mut self, v: u16) -> Result<(), core::fmt::Error> {
        self.write_fixed_size("0x")?;
        self.write_u16(v)
    }
    /// write an i16 to the output as a base-16 integer with leading `0x`, and leading `-` if the
    /// value is negative.
    ///
    /// there is no matching `std` formatter, so some examples here:
    /// ```text
    /// sink.write_prefixed_i16(-0x60); // writes `-0x60` to the sink
    /// sink.write_prefixed_i16(127); // writes `0x7f` to the sink
    /// sink.write_prefixed_i16(-128); // writes `-0x80` to the sink
    /// ```
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    fn write_prefixed_i16(&mut self, v: i16) -> Result<(), core::fmt::Error> {
        let v = if v < 0 {
            self.write_char('-')?;
            v.unsigned_abs()
        } else {
            v as u16
        };
        self.write_prefixed_u16(v)
    }
    /// write a u32 to the output as a base-16 integer.
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    fn write_u32(&mut self, v: u32) -> Result<(), core::fmt::Error> {
        write!(self, "{:x}", v)
    }
    /// write a u32 to the output as a base-16 integer with leading `0x`.
    ///
    /// this corresponds to the Rust format specifier `{#:x}` - see [`std::fmt::LowerHex`] for more.
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    fn write_prefixed_u32(&mut self, v: u32) -> Result<(), core::fmt::Error> {
        self.write_fixed_size("0x")?;
        self.write_u32(v)
    }
    /// write an i32 to the output as a base-32 integer with leading `0x`, and leading `-` if the
    /// value is negative.
    ///
    /// there is no matching `std` formatter, so some examples here:
    /// ```text
    /// sink.write_prefixed_i32(-0x60); // writes `-0x60` to the sink
    /// sink.write_prefixed_i32(127); // writes `0x7f` to the sink
    /// sink.write_prefixed_i32(-128); // writes `-0x80` to the sink
    /// ```
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    fn write_prefixed_i32(&mut self, v: i32) -> Result<(), core::fmt::Error> {
        let v = if v < 0 {
            self.write_char('-')?;
            v.unsigned_abs()
        } else {
            v as u32
        };
        self.write_prefixed_u32(v)
    }
    /// write a u64 to the output as a base-16 integer.
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    fn write_u64(&mut self, v: u64) -> Result<(), core::fmt::Error> {
        write!(self, "{:x}", v)
    }
    /// write a u64 to the output as a base-16 integer with leading `0x`.
    ///
    /// this corresponds to the Rust format specifier `{#:x}` - see [`std::fmt::LowerHex`] for more.
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    fn write_prefixed_u64(&mut self, v: u64) -> Result<(), core::fmt::Error> {
        self.write_fixed_size("0x")?;
        self.write_u64(v)
    }
    /// write an i64 to the output as a base-64 integer with leading `0x`, and leading `-` if the
    /// value is negative.
    ///
    /// there is no matching `std` formatter, so some examples here:
    /// ```text
    /// sink.write_prefixed_i64(-0x60); // writes `-0x60` to the sink
    /// sink.write_prefixed_i64(127); // writes `0x7f` to the sink
    /// sink.write_prefixed_i64(-128); // writes `-0x80` to the sink
    /// ```
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    fn write_prefixed_i64(&mut self, v: i64) -> Result<(), core::fmt::Error> {
        let v = if v < 0 {
            self.write_char('-')?;
            v.unsigned_abs()
        } else {
            v as u64
        };
        self.write_prefixed_u64(v)
    }

    /// enter a region inside which output corresponds to an immediate.
    fn span_start_immediate(&mut self) { }
    /// end a region where an immediate was written. see docs on [`DisplaySink`] for more.
    fn span_end_immediate(&mut self) { }

    /// enter a region inside which output corresponds to a register.
    fn span_start_register(&mut self) { }
    /// end a region where a register was written. see docs on [`DisplaySink`] for more.
    fn span_end_register(&mut self) { }

    /// enter a region inside which output corresponds to an opcode.
    fn span_start_opcode(&mut self) { }
    /// end a region where an opcode was written. see docs on [`DisplaySink`] for more.
    fn span_end_opcode(&mut self) { }

    /// enter a region inside which output corresponds to the program counter.
    fn span_start_program_counter(&mut self) { }
    /// end a region where the program counter was written. see docs on [`DisplaySink`] for more.
    fn span_end_program_counter(&mut self) { }

    /// enter a region inside which output corresponds to a number, such as a memory offset or
    /// immediate.
    fn span_start_number(&mut self) { }
    /// end a region where a number was written. see docs on [`DisplaySink`] for more.
    fn span_end_number(&mut self) { }

    /// enter a region inside which output corresponds to an address. this is a best guess;
    /// instructions like x86's `lea` may involve an "address" that is not, and arithmetic
    /// instructions may operate on addresses held in registers.
    ///
    /// where possible, the presence of this span will be informed by ISA semantics - if an
    /// instruction has a memory operand, the effective address calculation of that operand should
    /// be in an address span.
    fn span_start_address(&mut self) { }
    /// end a region where an address was written. the specifics of an "address" are ambiguous and
    /// best-effort; see [`DisplaySink::span_start_address`] for more about this. otherwise, see
    /// docs on [`DisplaySink`] for more about spans.
    fn span_end_address(&mut self) { }

    /// enter a region inside which output corresponds to a function address, or expression
    /// evaluating to a function address. this is a best guess; instructions like `call` may call
    /// to a non-function address, `jmp` may jump to a function (as with tail calls), function
    /// addresses may be computed via table lookup without semantic hints.
    ///
    /// where possible, the presence of this span will be informed by ISA semantics - if an
    /// instruction is like a "call", an address operand should be a `function` span. if other
    /// instructions can be expected to handle subroutine starting addresses purely from ISA
    /// semantics, address operand(s) should be in a `function` span.
    fn span_start_function_expr(&mut self) { }
    /// end a region where function address expression was written. the specifics of a "function
    /// address" are ambiguous and best-effort; see [`DisplaySink::span_start_function_expr`] for more
    /// about this. otherwise, see docs on [`DisplaySink`] for more about spans.
    fn span_end_function_expr(&mut self) { }
}

/// `FmtSink` can be used to adapt any `fmt::Write`-implementing type into a `DisplaySink` to
/// format an instruction while discarding all span information at zero cost.
pub struct FmtSink<'a, T: fmt::Write> {
    out: &'a mut T,
}

impl<'a, T: fmt::Write> FmtSink<'a, T> {
    pub fn new(f: &'a mut T) -> Self {
        Self { out: f }
    }
}

/// blanket impl that discards all span information, forwards writes to the underlying `fmt::Write`
/// type.
impl<'a, T: fmt::Write> DisplaySink for FmtSink<'a, T> { }

impl<'a, T: fmt::Write> fmt::Write for FmtSink<'a, T> {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.out.write_str(s)
    }
    fn write_char(&mut self, c: char) -> Result<(), core::fmt::Error> {
        self.out.write_char(c)
    }
    fn write_fmt(&mut self, f: fmt::Arguments) -> Result<(), core::fmt::Error> {
        self.out.write_fmt(f)
    }
}

#[cfg(feature = "alloc")]
mod instruction_text_sink {
    use core::fmt;

    use super::{DisplaySink, u8_to_hex};
    use crate::safer_unchecked::unreachable_kinda_unchecked;

    /// this is an implementation detail of yaxpeax-arch and related crates. if you are a user of the
    /// disassemblers, do not use this struct. do not depend on this struct existing. this struct is
    /// not stable. this struct is not safe for general use. if you use this struct you and your
    /// program will be eaten by gremlins.
    ///
    /// if you are implementing an instruction formatter for the yaxpeax family of crates: this struct
    /// is guaranteed to contain a string that is long enough to hold a fully-formatted instruction.
    /// because the buffer is guaranteed to be long enough, writes through `InstructionTextSink` are
    /// not bounds-checked, and the buffer is never grown.
    ///
    /// this is wildly dangerous in general use. the public constructor of `InstructionTextSink` is
    /// unsafe as a result. as used in `InstructionFormatter`, the buffer is guaranteed to be
    /// `clear()`ed before use, `InstructionFormatter` ensures the buffer is large enough, *and*
    /// `InstructionFormatter` never allows `InstructionTextSink` to exist in a context where it would
    /// be written to without being rewound first.
    ///
    /// because this opens a very large hole through which `fmt::Write` can become unsafe, incorrect
    /// uses of this struct will be hard to debug in general. `InstructionFormatter` is probably at the
    /// limit of easily-reasoned-about lifecycle of the buffer, which "only" leaves the problem of
    /// ensuring that instruction formatting impls this buffer is passed to are appropriately sized.
    ///
    /// this is intended to be hidden in docs. if you see this in docs, it's a bug.
#[doc(hidden)]
    pub struct InstructionTextSink<'buf> {
        buf: &'buf mut alloc::string::String
    }

    impl<'buf> InstructionTextSink<'buf> {
        // TODO: safety
        pub unsafe fn new(buf: &'buf mut alloc::string::String) -> Self {
            Self { buf }
        }
    }

    impl<'buf> fmt::Write for InstructionTextSink<'buf> {
        fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
            self.buf.write_str(s)
        }
        fn write_char(&mut self, c: char) -> Result<(), core::fmt::Error> {
            if cfg!(debug_assertions) {
                if self.buf.capacity() < self.buf.len() + 1 {
                    panic!("InstructionTextSink::write_char would overflow output");
                }
            }
            // SAFETY: `buf` is assumed to be long enough to hold all input, `buf` at `underlying.len()`
            // is valid for writing, but may be uninitialized.
            //
            // this function is essentially equivalent to `Vec::push` specialized for the case that
            // `len < buf.capacity()`:
            // https://github.com/rust-lang/rust/blob/be9e27e/library/alloc/src/vec/mod.rs#L1993-L2006
            unsafe {
                let underlying = self.buf.as_mut_vec();
                // `InstructionTextSink::write_char` is only used by yaxpeax-x86, and is only used to
                // write single ASCII characters. this is wrong in the general case, but `write_char`
                // here is not going to be used in the general case.
                if cfg!(debug_assertions) {
                    panic!("InstructionTextSink::write_char would truncate output");
                }
                let to_push = c as u8;
                // `ptr::write` here because `underlying.add(underlying.len())` may not point to an
                // initialized value, which would mean that turning that pointer into a `&mut u8` to
                // store through would be UB. `ptr::write` avoids taking the mut ref.
                underlying.as_mut_ptr().offset(underlying.len() as isize).write(to_push);
                // we have initialized all (one) bytes that `set_len` is increasing the length to
                // include.
                underlying.set_len(underlying.len() + 1);
            }
            Ok(())
        }
    }

    impl<'buf> DisplaySink for InstructionTextSink<'buf> {
        #[inline(always)]
        fn write_fixed_size(&mut self, s: &str) -> Result<(), core::fmt::Error> {
            if cfg!(debug_assertions) {
                if self.buf.capacity() < self.buf.len() + s.len() {
                    panic!("InstructionTextSink::write_fixed_size would overflow output");
                }
            }

            let buf = unsafe { self.buf.as_mut_vec() };
            let new_bytes = s.as_bytes();

            if new_bytes.len() == 0 {
                return Ok(());
            }

            if new_bytes.len() >= 16 {
                unsafe { unreachable_kinda_unchecked() }
            }

            unsafe {
                let dest = buf.as_mut_ptr().offset(buf.len() as isize);

                // this used to be enough to bamboozle llvm away from
                // https://github.com/rust-lang/rust/issues/92993#issuecomment-2028915232https://github.com/rust-lang/rust/issues/92993#issuecomment-2028915232
                // if `s` is not fixed size. somewhere between Rust 1.68 and Rust 1.74 this stopped
                // being sufficient, so `write_fixed_size` truly should only be used for fixed size `s`
                // (otherwise this is a libc memcpy call in disguise). for fixed-size strings this
                // unrolls into some kind of appropriate series of `mov`.
                dest.offset(0 as isize).write(new_bytes[0]);
                for i in 1..new_bytes.len() {
                    dest.offset(i as isize).write(new_bytes[i]);
                }

                buf.set_len(buf.len() + new_bytes.len());
            }

            Ok(())
        }
        unsafe fn write_lt_32(&mut self, s: &str) -> Result<(), fmt::Error> {
            if cfg!(debug_assertions) {
                if self.buf.capacity() < self.buf.len() + s.len() {
                    panic!("InstructionTextSink::write_lt_32 would overflow output");
                }
            }
            unsafe {
                super::append_helpers::append_string_lt_32_unchecked(&mut self.buf, s);
            }

            Ok(())
        }
        unsafe fn write_lt_16(&mut self, s: &str) -> Result<(), fmt::Error> {
            if cfg!(debug_assertions) {
                if self.buf.capacity() < self.buf.len() + s.len() {
                    panic!("InstructionTextSink::write_lt_16 would overflow output");
                }
            }

            unsafe {
                super::append_helpers::append_string_lt_16_unchecked(&mut self.buf, s);
            }

            Ok(())
        }
        unsafe fn write_lt_8(&mut self, s: &str) -> Result<(), fmt::Error> {
            if cfg!(debug_assertions) {
                if self.buf.capacity() < self.buf.len() + s.len() {
                    panic!("InstructionTextSink::write_lt_8 would overflow output");
                }
            }

            unsafe {
                super::append_helpers::append_string_lt_8_unchecked(&mut self.buf, s);
            }

            Ok(())
        }
        /// write a u8 to the output as a base-16 integer.
        ///
        /// this is provided for optimization opportunities when the formatted integer can be written
        /// directly to the sink (rather than formatted to an intermediate buffer and output as a
        /// followup step)
        #[inline(always)]
        fn write_u8(&mut self, mut v: u8) -> Result<(), core::fmt::Error> {
            if v == 0 {
                return self.write_fixed_size("0");
            }
            // we can fairly easily predict the size of a formatted string here with lzcnt, which also
            // means we can write directly into the correct offsets of the output string.
            let printed_size = ((8 - v.leading_zeros() + 3) >> 2) as usize;

            if cfg!(debug_assertions) {
                if self.buf.capacity() < self.buf.len() + printed_size {
                    panic!("InstructionTextSink::write_u8 would overflow output");
                }
            }

            let buf = unsafe { self.buf.as_mut_vec() };
            let new_len = buf.len() + printed_size;

            unsafe {
                buf.set_len(new_len);
            }
            let mut p = unsafe { buf.as_mut_ptr().offset(new_len as isize) };

            loop {
                let digit = v % 16;
                let c = u8_to_hex(digit as u8);
                unsafe {
                    p = p.offset(-1);
                    p.write(c);
                }
                v = v / 16;
                if v == 0 {
                    break;
                }
            }

            Ok(())
        }
        /// write a u16 to the output as a base-16 integer.
        ///
        /// this is provided for optimization opportunities when the formatted integer can be written
        /// directly to the sink (rather than formatted to an intermediate buffer and output as a
        /// followup step)
        #[inline(always)]
        fn write_u16(&mut self, mut v: u16) -> Result<(), core::fmt::Error> {
            if v == 0 {
                return self.write_fixed_size("0");
            }

            // we can fairly easily predict the size of a formatted string here with lzcnt, which also
            // means we can write directly into the correct offsets of the output string.
            let printed_size = ((16 - v.leading_zeros() + 3) >> 2) as usize;

            if cfg!(debug_assertions) {
                if self.buf.capacity() < self.buf.len() + printed_size {
                    panic!("InstructionTextSink::write_u16 would overflow output");
                }
            }

            let buf = unsafe { self.buf.as_mut_vec() };
            let new_len = buf.len() + printed_size;

            unsafe {
                buf.set_len(new_len);
            }
            let mut p = unsafe { buf.as_mut_ptr().offset(new_len as isize) };

            loop {
                let digit = v % 16;
                let c = u8_to_hex(digit as u8);
                unsafe {
                    p = p.offset(-1);
                    p.write(c);
                }
                v = v / 16;
                if v == 0 {
                    break;
                }
            }

            Ok(())
        }
        /// write a u32 to the output as a base-16 integer.
        ///
        /// this is provided for optimization opportunities when the formatted integer can be written
        /// directly to the sink (rather than formatted to an intermediate buffer and output as a
        /// followup step)
        #[inline(always)]
        fn write_u32(&mut self, mut v: u32) -> Result<(), core::fmt::Error> {
            if v == 0 {
                return self.write_fixed_size("0");
            }

            // we can fairly easily predict the size of a formatted string here with lzcnt, which also
            // means we can write directly into the correct offsets of the output string.
            let printed_size = ((32 - v.leading_zeros() + 3) >> 2) as usize;

            if cfg!(debug_assertions) {
                if self.buf.capacity() < self.buf.len() + printed_size {
                    panic!("InstructionTextSink::write_u32 would overflow output");
                }
            }

            let buf = unsafe { self.buf.as_mut_vec() };
            let new_len = buf.len() + printed_size;

            unsafe {
                buf.set_len(new_len);
            }
            let mut p = unsafe { buf.as_mut_ptr().offset(new_len as isize) };

            loop {
                let digit = v % 16;
                let c = u8_to_hex(digit as u8);
                unsafe {
                    p = p.offset(-1);
                    p.write(c);
                }
                v = v / 16;
                if v == 0 {
                    break;
                }
            }

            Ok(())
        }
        /// write a u64 to the output as a base-16 integer.
        ///
        /// this is provided for optimization opportunities when the formatted integer can be written
        /// directly to the sink (rather than formatted to an intermediate buffer and output as a
        /// followup step)
        #[inline(always)]
        fn write_u64(&mut self, mut v: u64) -> Result<(), core::fmt::Error> {
            if v == 0 {
                return self.write_fixed_size("0");
            }

            // we can fairly easily predict the size of a formatted string here with lzcnt, which also
            // means we can write directly into the correct offsets of the output string.
            let printed_size = ((64 - v.leading_zeros() + 3) >> 2) as usize;

            if cfg!(debug_assertions) {
                if self.buf.capacity() < self.buf.len() + printed_size {
                    panic!("InstructionTextSink::write_u64 would overflow output");
                }
            }

            let buf = unsafe { self.buf.as_mut_vec() };
            let new_len = buf.len() + printed_size;

            unsafe {
                buf.set_len(new_len);
            }
            let mut p = unsafe { buf.as_mut_ptr().offset(new_len as isize) };

            loop {
                let digit = v % 16;
                let c = u8_to_hex(digit as u8);
                unsafe {
                    p = p.offset(-1);
                    p.write(c);
                }
                v = v / 16;
                if v == 0 {
                    break;
                }
            }

            Ok(())
        }
    }
}
#[cfg(feature = "alloc")]
pub use instruction_text_sink::InstructionTextSink;


#[cfg(feature = "alloc")]
use crate::display::u8_to_hex;

#[cfg(feature = "alloc")]
use crate::safer_unchecked::unreachable_kinda_unchecked;

/// this [`DisplaySink`] impl exists to support somewhat more performant buffering of the kinds of
/// strings `yaxpeax-x86` uses in formatting instructions.
///
/// span information is discarded at zero cost.
#[cfg(feature = "alloc")]
impl DisplaySink for alloc::string::String {
    #[inline(always)]
    fn write_fixed_size(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.reserve(s.len());
        let buf = unsafe { self.as_mut_vec() };
        let new_bytes = s.as_bytes();

        if new_bytes.len() == 0 {
            unsafe { unreachable_kinda_unchecked() }
        }

        if new_bytes.len() >= 16 {
            unsafe { unreachable_kinda_unchecked() }
        }

        unsafe {
            let dest = buf.as_mut_ptr().offset(buf.len() as isize);

            // this used to be enough to bamboozle llvm away from
            // https://github.com/rust-lang/rust/issues/92993#issuecomment-2028915232
            // if `s` is not fixed size. somewhere between Rust 1.68 and Rust 1.74 this stopped
            // being sufficient, so `write_fixed_size` truly should only be used for fixed size `s`
            // (otherwise this is a libc memcpy call in disguise). for fixed-size strings this
            // unrolls into some kind of appropriate series of `mov`.
            dest.offset(0 as isize).write(new_bytes[0]);
            for i in 1..new_bytes.len() {
                dest.offset(i as isize).write(new_bytes[i]);
            }

            buf.set_len(buf.len() + new_bytes.len());
        }

        Ok(())
    }
    unsafe fn write_lt_32(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.reserve(s.len());

        unsafe {
            append_helpers::append_string_lt_32_unchecked(self, s);
        }

        Ok(())
    }
    unsafe fn write_lt_16(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.reserve(s.len());

        unsafe {
            append_helpers::append_string_lt_16_unchecked(self, s);
        }

        Ok(())
    }
    unsafe fn write_lt_8(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.reserve(s.len());

        unsafe {
            append_helpers::append_string_lt_8_unchecked(self, s);
        }

        Ok(())
    }
    /// write a u8 to the output as a base-16 integer.
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    #[inline(always)]
    fn write_u8(&mut self, mut v: u8) -> Result<(), core::fmt::Error> {
        if v == 0 {
            return self.write_fixed_size("0");
        }
        // we can fairly easily predict the size of a formatted string here with lzcnt, which also
        // means we can write directly into the correct offsets of the output string.
        let printed_size = ((8 - v.leading_zeros() + 3) >> 2) as usize;

        self.reserve(printed_size);

        let buf = unsafe { self.as_mut_vec() };
        let new_len = buf.len() + printed_size;

        unsafe {
            buf.set_len(new_len);
        }
        let mut p = unsafe { buf.as_mut_ptr().offset(new_len as isize) };

        loop {
            let digit = v % 16;
            let c = u8_to_hex(digit as u8);
            unsafe {
                p = p.offset(-1);
                p.write(c);
            }
            v = v / 16;
            if v == 0 {
                break;
            }
        }

        Ok(())
    }
    /// write a u16 to the output as a base-16 integer.
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    #[inline(always)]
    fn write_u16(&mut self, mut v: u16) -> Result<(), core::fmt::Error> {
        if v == 0 {
            return self.write_fixed_size("0");
        }
        // we can fairly easily predict the size of a formatted string here with lzcnt, which also
        // means we can write directly into the correct offsets of the output string.
        let printed_size = ((16 - v.leading_zeros() + 3) >> 2) as usize;

        self.reserve(printed_size);

        let buf = unsafe { self.as_mut_vec() };
        let new_len = buf.len() + printed_size;

        unsafe {
            buf.set_len(new_len);
        }
        let mut p = unsafe { buf.as_mut_ptr().offset(new_len as isize) };

        loop {
            let digit = v % 16;
            let c = u8_to_hex(digit as u8);
            unsafe {
                p = p.offset(-1);
                p.write(c);
            }
            v = v / 16;
            if v == 0 {
                break;
            }
        }

        Ok(())
    }
    /// write a u32 to the output as a base-16 integer.
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    #[inline(always)]
    fn write_u32(&mut self, mut v: u32) -> Result<(), core::fmt::Error> {
        if v == 0 {
            return self.write_fixed_size("0");
        }
        // we can fairly easily predict the size of a formatted string here with lzcnt, which also
        // means we can write directly into the correct offsets of the output string.
        let printed_size = ((32 - v.leading_zeros() + 3) >> 2) as usize;

        self.reserve(printed_size);

        let buf = unsafe { self.as_mut_vec() };
        let new_len = buf.len() + printed_size;

        unsafe {
            buf.set_len(new_len);
        }
        let mut p = unsafe { buf.as_mut_ptr().offset(new_len as isize) };

        loop {
            let digit = v % 16;
            let c = u8_to_hex(digit as u8);
            unsafe {
                p = p.offset(-1);
                p.write(c);
            }
            v = v / 16;
            if v == 0 {
                break;
            }
        }

        Ok(())
    }
    /// write a u64 to the output as a base-16 integer.
    ///
    /// this is provided for optimization opportunities when the formatted integer can be written
    /// directly to the sink (rather than formatted to an intermediate buffer and output as a
    /// followup step)
    #[inline(always)]
    fn write_u64(&mut self, mut v: u64) -> Result<(), core::fmt::Error> {
        if v == 0 {
            return self.write_fixed_size("0");
        }
        // we can fairly easily predict the size of a formatted string here with lzcnt, which also
        // means we can write directly into the correct offsets of the output string.
        let printed_size = ((64 - v.leading_zeros() + 3) >> 2) as usize;

        self.reserve(printed_size);

        let buf = unsafe { self.as_mut_vec() };
        let new_len = buf.len() + printed_size;

        unsafe {
            buf.set_len(new_len);
        }
        let mut p = unsafe { buf.as_mut_ptr().offset(new_len as isize) };

        loop {
            let digit = v % 16;
            let c = u8_to_hex(digit as u8);
            unsafe {
                p = p.offset(-1);
                p.write(c);
            }
            v = v / 16;
            if v == 0 {
                break;
            }
        }

        Ok(())
    }
}

#[cfg(feature="alloc")]
mod append_helpers {
    /// append `data` to `buf`, assuming `data` is less than 8 bytes and that `buf` has enough space
    /// remaining to hold all bytes in `data`.
    ///
    /// Safety: callers must ensure that `buf.capacity() - buf.len() >= data.len()`.
    #[inline(always)]
    pub unsafe fn append_string_lt_8_unchecked(buf: &mut alloc::string::String, data: &str) {
        // SAFETY: todo
        let buf = unsafe { buf.as_mut_vec() };
        let new_bytes = data.as_bytes();

        // should get DCE
        if new_bytes.len() >= 8 {
            unsafe { core::hint::unreachable_unchecked() }
        }

        unsafe {
            let dest = buf.as_mut_ptr().offset(buf.len() as isize);
            let src = new_bytes.as_ptr();

            let rem = new_bytes.len() as isize;

            // set_len early because there is no way to avoid the following asm!() writing that
            // same number of bytes into buf
            buf.set_len(buf.len() + new_bytes.len());

            core::arch::asm!(
                "8:",
                "cmp {rem:e}, 4",
                "jb 9f",
                "mov {buf:e}, dword ptr [{src} + {rem} - 4]",
                "mov dword ptr [{dest} + {rem} - 4], {buf:e}",
                "sub {rem:e}, 4",
                "jz 11f",
                "9:",
                "cmp {rem:e}, 2",
                "jb 10f",
                "mov {buf:x}, word ptr [{src} + {rem} - 2]",
                "mov word ptr [{dest} + {rem} - 2], {buf:x}",
                "sub {rem:e}, 2",
                "jz 11f",
                "10:",
                "cmp {rem:e}, 1",
                "jb 11f",
                "mov {buf:l}, byte ptr [{src} + {rem} - 1]",
                "mov byte ptr [{dest} + {rem} - 1], {buf:l}",
                "11:",
                src = in(reg) src,
                dest = in(reg) dest,
                rem = inout(reg) rem => _,
                buf = out(reg) _,
                options(nostack),
            );
        }
    }

    /// append `data` to `buf`, assuming `data` is less than 16 bytes and that `buf` has enough space
    /// remaining to hold all bytes in `data`.
    ///
    /// Safety: callers must ensure that `buf.capacity() - buf.len() >= data.len()`.
    #[inline(always)]
    pub unsafe fn append_string_lt_16_unchecked(buf: &mut alloc::string::String, data: &str) {
        // SAFETY: todo
        let buf = unsafe { buf.as_mut_vec() };
        let new_bytes = data.as_bytes();

        // should get DCE
        if new_bytes.len() >= 16 {
            unsafe { core::hint::unreachable_unchecked() }
        }

        unsafe {
            let dest = buf.as_mut_ptr().offset(buf.len() as isize);
            let src = new_bytes.as_ptr();

            let rem = new_bytes.len() as isize;

            // set_len early because there is no way to avoid the following asm!() writing that
            // same number of bytes into buf
            buf.set_len(buf.len() + new_bytes.len());

            core::arch::asm!(
                "7:",
                "cmp {rem:e}, 8",
                "jb 8f",
                "mov {buf:r}, qword ptr [{src} + {rem} - 8]",
                "mov qword ptr [{dest} + {rem} - 8], {buf:r}",
                "sub {rem:e}, 8",
                "jz 11f",
                "8:",
                "cmp {rem:e}, 4",
                "jb 9f",
                "mov {buf:e}, dword ptr [{src} + {rem} - 4]",
                "mov dword ptr [{dest} + {rem} - 4], {buf:e}",
                "sub {rem:e}, 4",
                "jz 11f",
                "9:",
                "cmp {rem:e}, 2",
                "jb 10f",
                "mov {buf:x}, word ptr [{src} + {rem} - 2]",
                "mov word ptr [{dest} + {rem} - 2], {buf:x}",
                "sub {rem:e}, 2",
                "jz 11f",
                "10:",
                "cmp {rem:e}, 1",
                "jb 11f",
                "mov {buf:l}, byte ptr [{src} + {rem} - 1]",
                "mov byte ptr [{dest} + {rem} - 1], {buf:l}",
                "11:",
                src = in(reg) src,
                dest = in(reg) dest,
                rem = inout(reg) rem => _,
                buf = out(reg) _,
                options(nostack),
            );
        }
    }

    /// append `data` to `buf`, assuming `data` is less than 32 bytes and that `buf` has enough space
    /// remaining to hold all bytes in `data`.
    ///
    /// Safety: callers must ensure that `buf.capacity() - buf.len() >= data.len()`.
    #[inline(always)]
    pub unsafe fn append_string_lt_32_unchecked(buf: &mut alloc::string::String, data: &str) {
        // SAFETY: todo
        let buf = unsafe { buf.as_mut_vec() };
        let new_bytes = data.as_bytes();

        // should get DCE
        if new_bytes.len() >= 32 {
            unsafe { core::hint::unreachable_unchecked() }
        }

        unsafe {
            let dest = buf.as_mut_ptr().offset(buf.len() as isize);
            let src = new_bytes.as_ptr();

            let rem = new_bytes.len() as isize;

            // set_len early because there is no way to avoid the following asm!() writing that
            // same number of bytes into buf
            buf.set_len(buf.len() + new_bytes.len());

            core::arch::asm!(
                "6:",
                "cmp {rem:e}, 16",
                "jb 7f",
                "mov {buf:r}, qword ptr [{src} + {rem} - 16]",
                "mov qword ptr [{dest} + {rem} - 16], {buf:r}",
                "mov {buf:r}, qword ptr [{src} + {rem} - 8]",
                "mov qword ptr [{dest} + {rem} - 8], {buf:r}",
                "sub {rem:e}, 16",
                "jz 11f",
                "7:",
                "cmp {rem:e}, 8",
                "jb 8f",
                "mov {buf:r}, qword ptr [{src} + {rem} - 8]",
                "mov qword ptr [{dest} + {rem} - 8], {buf:r}",
                "sub {rem:e}, 8",
                "jz 11f",
                "8:",
                "cmp {rem:e}, 4",
                "jb 9f",
                "mov {buf:e}, dword ptr [{src} + {rem} - 4]",
                "mov dword ptr [{dest} + {rem} - 4], {buf:e}",
                "sub {rem:e}, 4",
                "jz 11f",
                "9:",
                "cmp {rem:e}, 2",
                "jb 10f",
                "mov {buf:x}, word ptr [{src} + {rem} - 2]",
                "mov word ptr [{dest} + {rem} - 2], {buf:x}",
                "sub {rem:e}, 2",
                "jz 11f",
                "10:",
                "cmp {rem:e}, 1",
                "jb 11f",
                "mov {buf:l}, byte ptr [{src} + {rem} - 1]",
                "mov byte ptr [{dest} + {rem} - 1], {buf:l}",
                "11:",
                src = in(reg) src,
                dest = in(reg) dest,
                rem = inout(reg) rem => _,
                buf = out(reg) _,
                options(nostack),
            );
        }
    }
}
