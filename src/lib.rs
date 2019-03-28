extern crate num_traits;
extern crate termion;

use std::str::FromStr;

use std::fmt::{Debug, Display, Formatter};

use std::ops::{Add, Sub, AddAssign, SubAssign};

use std::rc::Rc;

use num_traits::identities;
use num_traits::{Bounded, WrappingAdd, WrappingSub, CheckedAdd, CheckedSub};

use termion::color;

                                                             // This is pretty wonk..
pub trait AddressDisplay {
    fn stringy(&self) -> String;
}

/*
 * TODO: this should be FromStr.
 * that would require newtyping address primitives, though
 *
 * this is not out of the question, BUT is way more work than
 * i want to put in right now
 *
 * this is one of those "clean it up later" situations
 */
pub trait AddrParse: Sized {
    type Err;
    fn parse_from(s: &str) -> Result<Self, Self::Err>;
}

pub trait Address where Self:
    Debug + Display + AddressDisplay +
    Copy + Clone + Sized +
    Ord + Eq + PartialEq + Bounded +
    Add<Output=Self> + Sub<Output=Self> +
    AddAssign + SubAssign +
    WrappingAdd + WrappingSub +
    CheckedAdd + CheckedSub +
    AddrParse +
    identities::One + identities::Zero {
    fn to_linear(&self) -> usize;

}
/*
impl <T> Address for T where T: Sized + Ord + Add<Output=Self> + From<u16> + Into<usize> {
    fn to_linear(&self) -> usize { *self.into() }
}
*/

impl AddrParse for usize {
    type Err = std::num::ParseIntError;
    fn parse_from(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            usize::from_str_radix(&s[2..], 16)
        } else {
            usize::from_str(s)
        }
    }
}

impl AddressDisplay for usize {
    fn stringy(&self) -> String {
        format!("{:#x}", self)
    }
}

impl AddrParse for u64 {
    type Err = std::num::ParseIntError;
    fn parse_from(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            u64::from_str_radix(&s[2..], 16)
        } else {
            u64::from_str(s)
        }
    }
}

impl AddressDisplay for u64 {
    fn stringy(&self) -> String {
        format!("{:#x}", self)
    }
}

impl AddrParse for u32 {
    type Err = std::num::ParseIntError;
    fn parse_from(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            u32::from_str_radix(&s[2..], 16)
        } else {
            u32::from_str(s)
        }
    }
}

impl AddressDisplay for u32 {
    fn stringy(&self) -> String {
        format!("{:#x}", self)
    }
}

impl AddrParse for u16 {
    type Err = std::num::ParseIntError;
    fn parse_from(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            u16::from_str_radix(&s[2..], 16)
        } else {
            u16::from_str(s)
        }
    }
}

impl AddressDisplay for u16 {
    fn stringy(&self) -> String {
        format!("{:#x}", self)
    }
}

impl Address for u16 {
    fn to_linear(&self) -> usize { *self as usize }
}

impl Address for u32 {
    fn to_linear(&self) -> usize { *self as usize }
}

impl Address for u64 {
    fn to_linear(&self) -> usize { *self as usize }
}

impl Address for usize {
    fn to_linear(&self) -> usize { *self }
}

pub trait Decodable where Self: Sized {
    fn decode<T: IntoIterator<Item=u8>>(bytes: T) -> Option<Self>;
    fn decode_into<T: IntoIterator<Item=u8>>(&mut self, bytes: T) -> Option<()>;
}

pub trait Arch {
    type Address: Address + Debug;
    type Instruction: Decodable + LengthedInstruction<Unit=Self::Address> + Debug;
    type Operand;
}

pub trait LengthedInstruction {
    type Unit;
    fn len(&self) -> Self::Unit;
    fn min_size() -> Self::Unit;
}

pub struct ColorSettings {
    arithmetic: color::Fg<&'static color::Color>,
    stack: color::Fg<&'static color::Color>,
    nop: color::Fg<&'static color::Color>,
    stop: color::Fg<&'static color::Color>,
    control: color::Fg<&'static color::Color>,
    data: color::Fg<&'static color::Color>,
    comparison: color::Fg<&'static color::Color>,
    invalid: color::Fg<&'static color::Color>,
    platform: color::Fg<&'static color::Color>,
    misc: color::Fg<&'static color::Color>,

    register: color::Fg<&'static color::Color>,

    number: color::Fg<&'static color::Color>,
    zero: color::Fg<&'static color::Color>,
    one: color::Fg<&'static color::Color>,
    minus_one: color::Fg<&'static color::Color>,

    function: color::Fg<&'static color::Color>,
    symbol: color::Fg<&'static color::Color>,
    address: color::Fg<&'static color::Color>
}

impl Default for ColorSettings {
    fn default() -> ColorSettings {
        ColorSettings {
            /*
            arithmetic: color::Fg(&color::LightYellow),
            stack: color::Fg(&color::Magenta),
            nop: color::Fg(&color::Blue),
            stop: color::Fg(&color::LightRed),
            control: color::Fg(&color::Green),
            data: color::Fg(&color::LightMagenta),
            comparison: color::Fg(&color::Yellow),
            invalid: color::Fg(&color::Red),
            platform: color::Fg(&color::Cyan),
            misc: color::Fg(&color::LightCyan),
            */
            arithmetic: color::Fg(&color::Yellow),
            stack: color::Fg(&color::Magenta),
            nop: color::Fg(&color::Blue),
            stop: color::Fg(&color::Red),
            control: color::Fg(&color::Red),
            data: color::Fg(&color::Yellow),
            comparison: color::Fg(&color::Yellow),
            invalid: color::Fg(&color::Red),
            platform: color::Fg(&color::Cyan),
            misc: color::Fg(&color::LightCyan),

            register: color::Fg(&color::Cyan),

            number: color::Fg(&color::White),
            zero: color::Fg(&color::White),
            one: color::Fg(&color::White),
            minus_one: color::Fg(&color::White),

            function: color::Fg(&color::LightGreen),
            symbol: color::Fg(&color::LightGreen),
            address: color::Fg(&color::Green)
        }
    }
}

impl ColorSettings {
    pub fn arithmetic_op(&self) -> &color::Fg<&'static color::Color> { &self.arithmetic }
    pub fn stack_op(&self) -> &color::Fg<&'static color::Color> { &self.stack }
    pub fn nop_op(&self) -> &color::Fg<&'static color::Color> { &self.nop }
    pub fn stop_op(&self) -> &color::Fg<&'static color::Color> { &self.stop }
    pub fn control_flow_op(&self) -> &color::Fg<&'static color::Color> { &self.control }
    pub fn data_op(&self) -> &color::Fg<&'static color::Color> { &self.data }
    pub fn comparison_op(&self) -> &color::Fg<&'static color::Color> { &self.comparison }
    pub fn invalid_op(&self) -> &color::Fg<&'static color::Color> { &self.invalid }

    pub fn register(&self) -> &color::Fg<&'static color::Color> { &self.register }
    pub fn number(&self) -> &color::Fg<&'static color::Color> { &self.number }
    pub fn zero(&self) -> &color::Fg<&'static color::Color> { &self.zero }
    pub fn one(&self) -> &color::Fg<&'static color::Color> { &self.one }
    pub fn minus_one(&self) -> &color::Fg<&'static color::Color> { &self.minus_one }
    pub fn address(&self) -> &color::Fg<&'static color::Color> { &self.address }
    pub fn symbol(&self) -> &color::Fg<&'static color::Color> { &self.symbol }
    pub fn function(&self) -> &color::Fg<&'static color::Color> { &self.function }
}

/*
 * can this be a derivable trait or something?
 */
/*
impl <T: Colorize> Display for T {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        self.colorize(None, fmt)
    }
}
*/

pub trait Colorize<T: std::fmt::Write> {
    fn colorize(&self, colors: Option<&ColorSettings>, out: &mut T) -> std::fmt::Result;
}

/*
 * and make this auto-derive from a ShowContextual impl?
 */
/*
impl <T, U> Colorize for T where T: ShowContextual<Ctx=U> {
    fn colorize(&self, colors: Option<&ColorSettings>, fmt: &mut Formatter) -> std::fmt::Result {
        self.contextualize(colors, None, fmt)
    }
}
*/

pub trait ShowContextual<Addr, Ctx: ?Sized, T: std::fmt::Write> {
    fn contextualize(&self, colors: Option<&ColorSettings>, address: Addr, context: Option<&Ctx>, out: &mut T) -> std::fmt::Result;
}

/*
impl <C: ?Sized, T: std::fmt::Write, U: Colorize<T>> ShowContextual<C, T> for U {
    fn contextualize(&self, colors: Option<&ColorSettings>, context: Option<&C>, out: &mut T) -> std::fmt::Result {
        self.colorize(colors, out)
    }
}
*/
