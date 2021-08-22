## yaxpeax-arch

[![crate](https://img.shields.io/crates/v/yaxpeax-arch.svg?logo=rust)](https://crates.io/crates/yaxpeax-arch)
[![documentation](https://docs.rs/yaxpeax-arch/badge.svg)](https://docs.rs/yaxpeax-arch)

shared traits for architecture definitions, instruction decoders, and related interfaces for instruction decoders from the yaxpeax project.

typically this crate is only interesting if you're writing code to operate on multiple architectures that all implement `yaxpeax-arch` traits. for example, [yaxpeax-dis](https://crates.io/crates/yaxpeax-dis) implements disassembly and display logic generic over the traits defined here, so adding a new decoder is usually only a one or two line addition.

`yaxpeax-arch` has several crate features, which implementers are encouraged to also support:
* `std`: opt-in for `std`-specific support - in this crate, `std` enables a [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html) requirement on `DecodeError`, allowing users to `?`-unwrap decode results.
* `colors`: enables (optional) [`crossterm`](https://docs.rs/crossterm/latest/crossterm/)-based ANSI colorization. default coloring rules are defined by [`ColorSettings`](https://docs.rs/yaxpeax-arch/latest/yaxpeax_arch/struct.ColorSettings.html), when enabled.
* `address-parse`: enable a requirement that `yaxpeax_arch::Address` be parsable from `&str`. this is useful for use cases that, for example, read addresses from humans.
* `use-serde`: enable [`serde`](https://docs.rs/serde/latest/serde/) serialization and deserialization bounds for types like `Address`.

with all features disabled, `yaxpeax-arch`'s only direct dependency is `num-traits`, and is suitable for `#![no_std]` usage.

### design

`yaxpeax-arch` has backwards-incompatible changes from time to time, but there's not much to make incompatible. the main benefit of this crate is the [`Arch`](https://docs.rs/yaxpeax-arch/latest/yaxpeax_arch/trait.Arch.html) trait, for other libraries to build architecture-agnostic functionality.

nontrivial additions to `yaxpeax-arch` should include some discussion summarized by an addition to the crate [`docs/`](https://github.com/iximeow/yaxpeax-arch/tree/no-gods-no-/docs). you may ask, "where does discussion happen?", and the answer currently is in my (iximeow's) head, or various discord/irc/discord/email conversations. if there's need in the future, `yaxpeax` may develop a more consistent process.

`yaxpeax-arch` intends to support ad-hoc development of architecture support. maintainers of various architectures' crates may not want to implement all available interfaces to a complete level of detail, and must not be required to. incomplete implementations may be an issue for downstream users, but library quality is mediated by human conversation, not `yaxpeax-arch` interfaces. extensions to these fundamental definitions should be considerate of partial and incomplete implementations.

### implementations

there are numerous architectures for which decoders are implemented, at varying levels of completion. now and in the future, they will be enumerated here:

| symbol | meaning |
| ------ | ------- |
| 🥳 | complete, reliable |
| ⚠️| "complete", likely has gaps |
| 🚧 | incomplete |
| ❓ | unimplemented |


| architecture | library | decode | tests | benchmarks | note |
| ------------ | ------- | ------ | ----- | ---------- | ---- |
| `x86_64` | [yaxpeax-x86](https://www.github.com/iximeow/yaxpeax-x86) | 🥳 | 🥳 | 🥳 | |
| `x86:32` | [yaxpeax-x86](https://www.github.com/iximeow/yaxpeax-x86) | 🥳 | 🥳 | ❓ | sse and sse2 support cannot be disabled |
| `x86:16` | [yaxpeax-x86](https://www.github.com/iximeow/yaxpeax-x86) | 🥳 | 🥳 | ❓ | instructions above the 8086 or 286 cannot be disabled |
| `ia64` | [yaxpeax-ia64](https://www.github.com/iximeow/yaxpeax-ia64) | 🥳 | ⚠️ | ❓ | lack of a good oracle has complicated testing |
| `armv7` | [yaxpeax-arm](https://www.github.com/iximeow/yaxpeax-arm) | 🚧 | 🚧 | ❓ | NEON is not yet supported |
| `armv8` | [yaxpeax-arm](https://www.github.com/iximeow/yaxpeax-arm) | 🚧 | 🚧 | ❓ | a32 decoding is not yet supported, NEON is not supported |
| `m16c` | [yaxpeax-m16c](https://www.github.com/iximeow/yaxpeax-m16c) | ⚠️ | 🚧 | ❓ | |
| `mips` | [yaxpeax-mips](https://www.github.com/iximeow/yaxpeax-mips) | 🚧 | 🚧 | ❓ | |
| `msp430` | [yaxpeax-msp430](https://www.github.com/iximeow/yaxpeax-msp430) | 🚧 | 🚧 | ❓ | |
| `pic17` | [yaxpeax-pic17](https://www.github.com/iximeow/yaxpeax-pic17) | 🚧 | 🚧 | ❓ | |
| `pic18` | [yaxpeax-pic18](https://www.github.com/iximeow/yaxpeax-pic18) | 🚧 | 🚧 | ❓ | |
| `pic24` | [yaxpeax-pic24](https://www.github.com/iximeow/yaxpeax-pic24) | ❓ | ❓ | ❓ | exists, but only decodes `NOP` |
| `sm83` | [yaxpeax-sm83](https://www.github.com/iximeow/yaxpeax-sm83) | 🥳 | 🚧 | ❓ | |
| `avr` | [yaxpeax-avr](https://github.com/The6P4C/yaxpeax-avr) | 🥳 | 🚧 | ❓ | contributed by [@the6p4c](https://twitter.com/The6P4C)! |
| `sh`/`sh2`/`j2`/`sh3`/`sh4` | [yaxpeax-superh](https://git.sr.ht/~nabijaczleweli/yaxpeax-superh) | 🥳 | 🚧 | ❓ | contributed by [наб](https://nabijaczleweli.xyz) |
| `MOS 6502` | [yaxpeax-6502](https://github.com/cr1901/yaxpeax-6502) | ⚠️ | ❓ | ❓ | contributed by [@cr1901](https://www.twitter.com/cr1901) |
| `lc87` | [yaxpeax-lc87](https://www.github.com/iximeow/yaxpeax-lc87) | 🥳 | ⚠️ | ❓ | |

#### feature support

`yaxpeax-arch` defines a few typically-optional features that decoders can also implement, in addition to simple `(bytes) -> instruction` decoding. these are `yaxpeax-arch` traits (or collections thereof) which architectures implement, not crate features.

`description_spans`: implementation of [`AnnotatingDecoder`](https://docs.rs/yaxpeax-arch/latest/yaxpeax_arch/trait.AnnotatingDecoder.html), to decode instructions with bit-level details of what incoming bitstreams mean.
`contextualize`: implementation of [`ShowContextual`](https://docs.rs/yaxpeax-arch/latest/yaxpeax_arch/trait.ShowContextual.html), to display instructions with user-defined information in place of default instruction data. typically expected to show label names instead of relative branch addresses. **i do not recommend implementing this trait**, it needs significant reconsideration.

| architecture | `description_spans` | `contextualize` |
| ------------ | ------------------- | --------------- |
| `x86_64` | 🥳 | ❓ |
| `ia64` | ⚠️ | ❓ |
| `msp430` | 🥳 | ❓ |

### mirrors

the canonical copy of `yaxpeax-arch` is at [https://git.iximeow.net/yaxpeax-arch](https://git.iximeow.net/yaxpeax-arch).

`yaxpeax-arch` is also mirrored on GitHub at [https://www.github.com/iximeow/yaxpeax-arch](https://www.github.com/iximeow/yaxpeax-arch).
