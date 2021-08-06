## yaxpeax-arch

[![crate](https://img.shields.io/crates/v/yaxpeax-arch.svg?logo=rust)](https://crates.io/crates/yaxpeax-arch)
[![documentation](https://docs.rs/yaxpeax-arch/badge.svg)](https://docs.rs/yaxpeax-arch)

shared traits for architecture definitions, instruction decoders, and related interfaces for instruction decoders from the yaxpeax project.

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

#### features

`yaxpeax-arch` defines a few typically-optional features that decoders can also implement, in addition to simple `(bytes) -> instruction` decoding. these are not crate features, but `yaxpeax-arch` trait impls or collections thereof.

`description_spans`: implementation of [`AnnotatingDecoder`](https://docs.rs/yaxpeax-arch/latest/yaxpeax_arch/trait.AnnotatingDecoder.html), to decode instructions with bit-level details of what incoming bitstreams mean.
`contextualize`: implementation of [`ShowContextual`](https://docs.rs/yaxpeax-arch/latest/yaxpeax_arch/trait.ShowContextual.html), to display instructions with user-defined information in place of default instruction data. typically expected to show label names instead of relative branch addresses. **i do not recommend implementing this trait**, it needs significant reconsideration.

| architecture | `description_spans` | `contextualize` |
| ------------ | ------------------- | --------------- |
| `x86_64` | 🥳 | ❓ |
| `ia64` | 🥳 | ❓ |
| `msp430` | 🥳 | ❓ |

### mirrors

the canonical copy of `yaxpeax-arch` is at [https://git.iximeow.net/yaxpeax-arch](https://git.iximeow.net/yaxpeax-arch).

`yaxpeax-arch` is also mirrored on GitHub at [https://www.github.com/iximeow/yaxpeax-arch](https://www.github.com/iximeow/yaxpeax-arch).
