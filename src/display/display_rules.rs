/// `ArchDisplayRules` allows client code to control some aspects of instruction formatting. there
/// are relatively few elements of instruction formatting that generalize across architectures, so
/// the expected use of this trait is with additional architecture-specific `DisplayRules` traits
///
/// additionally, methods on `ArchDisplayRules` all have default bodies that correspond to "do the
/// default formatting behavior".
///
/// ## interaction with `DisplaySink` and spans
///
/// `DisplaySink` has a notion of spans, and as `ArchDisplayRules` (and architecture-specific
/// `DisplayRules`) receive a `&mut impl DisplaySink` a natural question is: who is responsible for
/// opening and closing spans?
///
/// `DisplayRules` implementations must open and close spans, if participating in display spans at
/// all, callers of `DisplayRules` should prefer to not open spans in anticipation of expected
/// formatting in a called formatter. for some fields, the caller may not anticipate a formatter's
/// interpretation; an "immediate" might be a selector for an x86 `wrmsr` which a `DisplayRules`
/// may print as a symbolic expression rather than an integer. likewise, memory accesses may become
/// symbolicated if address information is available, and branch targets may become symbols or
/// other non-address expressions.
///
/// for these reasons, `DisplayRules` must be entrusted with the responsibility to open and close
/// spans that accurately describe the reported data.
pub trait ArchDisplayRules<A: crate::Arch, S: crate::display::DisplaySink + ?Sized> {
    /// should instructions be printed in their most manual-friendly alias forms?
    ///
    /// for some architectures (x86) there are no aliases to be shown, where other (typically RISC)
    /// architectures rely heavily on assembler aliases to read "normally".
    ///
    /// some ARM/A32 examples:
    /// * `LSL` can become "`MOV`", like `lsl r1, r2, 0` -> `mov r1, r2`
    /// * `LDR` can become "`POP`", like `ldr r4, [r13, 4]!` -> `pop r4`
    ///
    /// or some POWER ISA examples:
    /// * `OR` can become "`MR`", like `or r2, r3, r3` -> `mr r2, r3`
    /// * `ORI` can become "`NOP`", like `ori r1, r1, 0` -> `nop`
    /// * `ADDI` can become "`LI`", like `add r1, r0, 0x1234` -> `li r1, 0x1234`
    /// * `ADDI` can also become "`SUBI`", like `addi r1, r2, -0x1234` -> `subi r1, r2, 0x1234`
    fn display_aliases(&self) -> bool {
        true
    }

    /// where in program space is the instruction being formatted?
    ///
    /// if this returns `None`, yaxpeax crates typically print address-relative information as
    /// `$ + offset` or `$ - offset`, where `$` is the address the offset is relative from. that
    /// typically means "the address of the start of the next instruction", but some architectures
    /// have more complex descriptions of branch offsets - ARM adds two instruction lengths, POWER
    /// adds zero instruction lengths! as `None` prohibits resolving address-relative fields,
    /// when it is returned, some fields and addresses may not reach [`Self::emit_address`].
    ///
    /// if this returns `Some`, address-relative fields can typically be resolved to a fixed
    /// address. in most cases address-sensitive fields will at least result in a call to
    /// [`Self::emit_address`].
    fn instr_addr(&self) -> Option<A::Address> {
        None
    }

    /// write an address out to the provided `DisplaySink`.
    ///
    /// the kinds of addresses emitted here are possibly absolute memory addresses (the `0x1234` in
    /// an x86 `dword ptr [0x1234]`), a direct branch address (the `0xfe006338` in a POWER
    /// `bla 0xfe006338`), or a normalized PC-relative value if [`Self::instr_addr`] returns
    /// `Some`.
    ///
    /// in that last case, the provided address is the field's relative offset plus
    /// `Self::instr_addr()`, as well the contribution from the instruction's length (if any). that
    /// is to say, a 5-byte x86 `call` with offset `0x1000` at address `0x20000` will get an
    /// address of `0x21005`. a POWER `bl` with offset `0x1000` at address `0x20000` will get an
    /// address of `0x21000`.
    ///
    /// "is this field an address" is determined by decoder crates in an instruction-local manner.
    /// using x86 as an example, `mov rax, 0x1234; mov rbx, [rax]` would consider `0x1234` an
    /// immediate, not an address, even though a more context-aware analysis would know that
    /// immediate is later used as an address. contextual analyses like this are not handled by
    /// yaxpeax decoder crates.
    ///
    /// `Ok(true)` indicates that the address has been expressed, however this `DisplayRules` does
    /// it, and `Ok(false)` indicates that the library's default formatting for `addr` should still
    /// be done.  returning an `Err` will abort printing the corresponding instruction.
    fn emit_address(&self, addr: A::Address, s: &mut S) -> Result<bool, core::fmt::Error> {
        let _ = addr;
        let _ = s;
        Ok(false)
    }
}
