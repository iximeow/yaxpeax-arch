## `DescriptionSink`

most architectures' machine code packs interesting meanings into specific bit fields, and one of the more important tasks of the yaxpeax decoders is to unpack these into opcodes, operands, and other instruction data for later use. in the worst case, some architectures - typically interpreted bytecodes - do less bit-packing and simply map bytes to instructions.

the yaxpeax decoders' primary role is to handle this unpacking into user code-friendly structs. i want decoders to be able to report the meaning of bitfields too, so user code can mark up bit streams.

implementing this capability should (borderline-"must") not regress performance for decoders that do not use it. as a constraint, this is surprisingly restrictive!

a. it rules out a parameter to [`Decoder::decode_into`](https://docs.rs/yaxpeax-arch/0.2.5/yaxpeax_arch/trait.Decoder.html#tymethod.decode_into): an ignored or unused parameter can still change how `decode_into` inlines.  
b. it rules out extra state on `Decoder` impls: writing to an unread `Vec` is still extra work at decode time.  

decoders other than x86 are less performance-sensitive, so **light** regressions in performance may be tolerable.

i would also like to:

c. not require decoders implement this to participate in code analysis [`yaxpeax-core`](https://github.com/iximeow/yaxpeax-core/) provides.  
d. re-use existing decode logic -- requiring myself and other decoder authors to write everything twice would be miserable.  

the point `c` suggests not adding this capability to existing traits. taken together, these constraints point towards a _new_ trait that _could_ be implemented as an independent copy of decode logic, like:

```rust
trait AnnotatingDecoder<A: Arch + ?Sized> {
    fn decode_with_annotation<
        T: Reader<A::Address, A::Word>,
    >(&mut self, inst: &mut A::Instruction, words: &mut T) -> Result<(), A::DecodeError>;
}
```

but for implementations, it's easiest to tack this onto an existing `Arch`'s `InstDecoder`. point `b` means no new state, so wherever details about a span of bits are recorded, it should be an additional `&mut` parameter. then, if that parameter is an impl of some `Sink` trait, `yaxpeax_arch` can provide a no-op implementation of the `Sink` and let call sites be eliminated for non-annotating decodes.

taken together, this ends up adding three traits:

```rust
pub trait DescriptionSink<Descriptor> {
    fn record(&mut self, start: u32, end: u32, description: Descriptor);
}

pub trait FieldDescription {
    fn id(&self) -> u32;
}

pub trait AnnotatingDecoder<A: Arch + ?Sized> {
    type FieldDescription: FieldDescription + Clone + Display + PartialEq;

    fn decode_with_annotation<
        T: Reader<A::Address, A::Word>,
        S: DescriptionSink<Self::FieldDescription>
    >(&self, inst: &mut A::Instruction, words: &mut T, sink: &mut S) -> Result<(), A::DecodeError>;
}
```

where `FieldDescription` lets callers that operate generically over spans do *something* with them. implementations can use `id` to tag descriptions that should be ordered together, regardless of the actual order the decoder reports them in. for some architectures, fields parsed later in decoding may influence the understanding of earlier fields, so reporting spans in `id`-order up front is an unreasonable burden. consider an x86 instruction, `660f6ec0` - the leading `66` is an operand-size override, but only after reading `0f6e` is it known that that prefix is changing the operands from `mm`/`dword` registers to `xmm`/`qword` registers. in fact this is only known _after_ reporting the opcode of `0f6e`, too.

`start` and `end` are bit offsets where a `description` applies. `description`s can overlap in part, or in full. exact bit order is known only by the architecture being decoded; is the order `0-7,8-15,16-23,24-31`, `7-0,15-8,23-16,31-24`, or something else? i'm not sure trying to encode that in `yaxpeax-arch` traits is useful right now. `start` and `end` are `u32` because in my professional opinion, `u16` is cursed, `u8` isn't large enough, and `u32` is the next smallest size. `id()` returns a `u32` because i never want to think of `id` space constraints; even if `id` encoded a `major.minor`-style pair of ordering components, the most constrained layout would be `u16.u16` for at most 65536 values in major or minor. that's a big instruction.

### implementation

i've added WIP support for span reporting to `msp430`, `ia64`, and `x86` decoders. i extended `yaxpeax-dis` to [make pretty lines](https://twitter.com/iximeow/status/1423930207614889984). more could be said about that; `id`-order is expected to be, roughtly, the order an instruction is decoded. some instructions sets keep the "first" bits as the low-order bits, some others use the higher bits first. so respecting `id`-order necessarily means some instruction sets will have fields "backwards" and make lines extra confusing.

decoders probably ought to indicate boundaries for significant parts of decoding, lest large instructions [like itanium](https://twitter.com/iximeow/status/1424092536071618561) be a nebulous mess. maybe `FieldDescription` could have an `is_separator()` to know when an element (and its bit range) indicates the end of part of an instruction?

for the most part, things work great. `yaxpeax-x86` had a minor performance regression. tracking it down wasn't too bad: the first one was because `sink` is a fifth argument for a non-inlined function. at this point most ABIs start spilling to memory. so an unused `sink` caused an extra stack write. this was a measurable overhead. the second regression was again pretty simple looking at `disas-bench` builds:

```sh
diff \
  ` # a typical non-formatting build, from cratesio yaxpeax-x86 1.0.4 ` \
  <(objdump -d bench-yaxpeax-no-fmt | grep -o ' .*long_mode.*>:')
  ` # a non-formatting build, from the local patch of yaxpeax-x86 with annotation reported to a no-op sink ` \
  <(objdump -d bench-yaxpeax-no-fmt-no-annotation | grep -o ' .*long_mode.*>:')
```

the entire diff output:
```diff
>  <_ZN11yaxpeax_x869long_mode8read_sib17hdc339ef7a182098aE>:
```

indeed, [`read_sib`](https://github.com/iximeow/yaxpeax-x86/blob/4371ed02ac30cb56ec4ddbf60c87e85c183d860b/src/long_mode/mod.rs#L5769-L5770) is not written as `inline(always)`, so it's possible this might not get inlined sometimes. since the only difference to `read_sib` is an extra parameter, for which all calls are no-ops that ignore arguments, i'm surprised to see the change, anyway. adding `#[inline(always)]` to `read_sib` returned `yaxpeax-x86` to "same-as-before" decode throughput.

in the process, i found a slight optimization for `read_sib` that removed a few extra branches from the function. the scrutiny was good after all.

### conclusion

in summary, it works. it doesn't slow down callers that don't need spans of information. decoders can implement it optionally and at their leisure, without being ineligible for analysis-oriented libraries.

this is almost certainly going to be in `yaxpeax-arch 0.2.6` with implementations trickling into decoders whenever it seems like fun.
