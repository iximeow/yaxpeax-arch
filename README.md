## yaxpeax-arch

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
| `x86_64` | yaxpeax-x86 | ⚠️| 🚧 | 🚧 | incomplete operand decoding, may incorrectly accept long instructions |
| `x86:32` | yaxpeax-x86 | ❓ | ❓ | ❓ | should share most but not all implementation with `x86_64` |
| `x86:16` | yaxpeax-x86 | ❓ | ❓ | ❓ | should share most but not all implementation with `x86:32` |
| `armv7` | yaxpeax-arm | 🚧 | 🚧 | ❓ | |
| `armv8` | yaxpeax-arm | 🚧 | 🚧 | ❓ | |
| `m16c` | yaxpeax-m16c | ⚠️ | 🚧 | ❓ | |
| `mips` | yaxpeax-mips | 🚧 | 🚧 | ❓ | |
| `msp430` | yaxpeax-msp430 | 🚧 | 🚧 | ❓ | |
| `pic17` | yaxpeax-pic17 | 🚧 | 🚧 | ❓ | |
| `pic18` | yaxpeax-pic18 | 🚧 | 🚧 | ❓ | |
| `pic24` | yaxpeax-pic24 | ❓ | ❓ | ❓ | exists, but only decodes `NOP` |

### mirrors

the canonical copy of `yaxpeax-arch` is at [https://git.iximeow.net/yaxpeax-arch](https://git.iximeow.net/yaxpeax-arch).

`yaxpeax-arch` is also mirrored on GitHub at [https://www.github.com/iximeow/yaxpeax-x86](https://www.github.com/iximeow/yaxpeax-arch).

### ! user beware !
these interfaces will almost certainly move and change. the version number is `0.0.1` and i mean it with every fiber of my being.
