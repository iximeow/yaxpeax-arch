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

### mirrors

the canonical copy of `yaxpeax-arch` is at [https://git.iximeow.net/yaxpeax-arch](https://git.iximeow.net/yaxpeax-arch).

`yaxpeax-arch` is also mirrored on GitHub at [https://www.github.com/iximeow/yaxpeax-arch](https://www.github.com/iximeow/yaxpeax-arch).
