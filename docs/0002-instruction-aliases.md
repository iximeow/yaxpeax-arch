## instruction aliases

many architectures, especially RISC architectures, have a notion of
"instruction aliases". these are instructions that are some simplified
description of a particular combination of operands to some other instruction.

probably the most common alias is `X |= X` being `nop`, however an architecture
happens to describe it. that may be a three-register `or` in some
architectures, a two-register `or` in others, a register/register/immediate
with 0 in others. of course, in 64-bit x86 specifically the opcode which
aliased to `nop` in 16/32-bit modes was [made into a proper
instruction](https://www.pagetable.com/?p=1216).

there is a choice of representation for these instructions, but the trade-offs
point in opposite directions. the options i see look something like:

### decode instructions into the "highest level" forms

if a manual says that "add a, b, -1" may be written as "sub a, b, 1", then
decode that case as `sub`. this might imply a "sub" opcode when the instruction
set itself, and the hardware, doesn't have one.

i've generally avoided taking this approach because the first goal of
disassemblers i write tends to be (if not for pure love of the game) some kind
of program analysis, where i care about some operations happening or not. by
promoting aliases into proper "opcodes", categories of instructions are larger
and sometimes can be harder to cross-reference with manuals. documentation gets
a little more annoying for the benefit of "more match arms" or "matching on
multiple Opcode variants instead of just one".

### decode into the "low level" form, but have Display compute aliases

if it's useful for Opcode to roughly conform to the hardware's instruction
set, then that's what the library will do. practically speaking, though,
there's plenty of value to the standard aliases in an instruction set. for one,
other disassemblers usually show them, and assemblers usually accept them,
which means that comparing with other tools will want the aliased forms of
instructions. the aliases themselves are described for a reason; it is more
natural to read "sub with immediate" than "add with negative immediate", so
tools both accept and emit these forms for good reason!

so, to try fitting between these competing goals, yaxpeax decoders often take
the approach that "`Opcode` is what the hardware does, `Display` compiles that
into human-friendly forms". this is an awkward middle ground though: what if a
library user *wants* the aliased forms of instructions? you must `Display` the
instruction (not format it yourself! because the alias calculation is not an
available API), and then re-parse it, or otherwise do substring matching
grossness to get the deed done.

it's not a great interface, and it feels rude to demand that library users be
reduced to `Display` and substring matching, when the premise of yaxpeax is
"good program analysis tooling".

and worse, with some of the more recent work in the yaxpeax extended universe,
there's increasing amounts of flexibility around instruction displaying. this
leaves the alias calculation in various `Display` impls in a bad spot: it needs
to at least be extracted out and made generic over some crate-specific
`DisplayStyle` or `DisplayRules, and what if you want controlled formatting
_without_ alias calcluation? hopefully there's a `DisplayRules` with an
`emit_instruction` which lets you take over the printing, but at that point i'm
asking you to write a `Display` impl yourself in more words.

### decode into low-level forms, leave aliases up to users

yaxpeax could of course take a principled stance and say "this decodes what the
hardware supports, end of discussion". again, this feels rude and somewhat
petty of a position. the libraries can do better, so i try to make them do
better.

## yax can do better!

relatively recently i made x86 formatting more configurable in support of both
MASM-style formatting and (finally) figuring out an approach to symbolizing
addresses in instructions. this ended up at [`trait DisplayRules`][display_rules]
and [`Instruction::display_rules`][inst_display] to get a formattable item.

by making display machinery generic over `R: DisplayRules`, the overhead of
simple display rules (say, formatting immediates as `01ABh` rather than
`0x1ab`) is much less than the code being littered with indirect calls. for the
"low" price of monomorphization and the attendant const evaluation, the
resulting function is effectively a specialization of `Display` with respect to
a set of display rules. cool!

this at least is a hint towards one option for instruction alias handling: by
convention, `DisplayRules` could include a `fn display_aliases(&self) -> bool`
which controls if a Display impl does or doesn't compute aliases. when
optimizing for total code size or undecorated instructions-into-text, turn off
alias calculation and save some number of kilobytes and cycles..

## but structured processing is valuable

handling this with `DisplayRules` and formatting is better than *nothing*, but
it doesn't help users that want to see an `Opcode::Add`, `Opcode::Sub`, or,
say, an ARM `Opcode::Pop`.

what keeps `Arch::Decoder` from having a similar switch? for programmatic
users, say an emulator handling all the `Opcode` variants to cover a CPU's
behavior, it can be counterproductive for `Opcode` to have variants that are
never actually produced. so, perhaps the switch really produces an alternate
`Arch::Decoder` which has a different `Opcode` type that covers additional
alias-only opcodes? then, an end-user never interested in custom handling of
aliased instructions doesn't need to spend the cycles to compute aliases, but a
`Display` impl can still by default choose to compute them. and even here, a
`DisplayRules` setting can opt out of that alias calcluation, which could be
implemented as, effectively,
```
if rules.show_aliases {
    rules.display(Instruction::compute_alias())?;
}
```

so. this is the plan. lets see how it goes!
