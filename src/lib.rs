extern crate num_traits;
extern crate termion;
#[cfg(feature="use-serde")]
extern crate serde;

use std::str::FromStr;
use std::hash::Hash;

use std::fmt::{Debug, Display, Formatter};

use std::ops::{Add, Sub, AddAssign, SubAssign};

use num_traits::identities;
use num_traits::{Bounded, WrappingAdd, WrappingSub, CheckedAdd, CheckedSub};

use termion::color;

#[cfg(feature="use-serde")]
use serde::{Serialize, Deserialize};

pub mod display;
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

#[cfg(feature="use-serde")]
pub trait Address where Self:
    Serialize + for<'de> Deserialize<'de> +
    Debug + Display + AddressDisplay +
    Copy + Clone + Sized +
    Ord + Eq + PartialEq + Bounded +
    Add<Output=Self> + Sub<Output=Self> +
    AddAssign + SubAssign +
    WrappingAdd + WrappingSub +
    CheckedAdd + CheckedSub +
    AddrParse +
    Hash +
    identities::One + identities::Zero {
    fn to_linear(&self) -> usize;

}
#[cfg(not(feature="use-serde"))]
pub trait Address where Self:
    Debug + Display + AddressDisplay +
    Copy + Clone + Sized +
    Ord + Eq + PartialEq + Bounded +
    Add<Output=Self> + Sub<Output=Self> +
    AddAssign + SubAssign +
    WrappingAdd + WrappingSub +
    CheckedAdd + CheckedSub +
    AddrParse +
    Hash +
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

#[cfg(feature="use-serde")]
pub trait Arch {
    type Address: Address + Debug + Hash + PartialEq + Eq + Serialize + for<'de> Deserialize<'de>;
    type Instruction: Decodable + LengthedInstruction<Unit=Self::Address> + Debug;
    type Operand;
}

#[cfg(not(feature="use-serde"))]
pub trait Arch {
    type Address: Address + Debug + Hash + PartialEq + Eq;
    type Instruction: Decodable + LengthedInstruction<Unit=Self::Address> + Debug;
    type Operand;
}

pub trait LengthedInstruction {
    type Unit;
    fn len(&self) -> Self::Unit;
    fn min_size() -> Self::Unit;
}

#[cfg(feature="use-serde")]
impl Serialize for ColorSettings {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("ColorSettings", 0)?;
        s.end()
    }
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
    program_counter: color::Fg<&'static color::Color>,

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
            platform: color::Fg(&color::LightBlue),
            misc: color::Fg(&color::LightCyan),

            register: color::Fg(&color::Cyan),
            program_counter: color::Fg(&color::Red),

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

pub trait YaxColors {
    fn arithmetic_op<T: Display>(&self, t: T) -> Colored<T>;
    fn stack_op<T: Display>(&self, t: T) -> Colored<T>;
    fn nop_op<T: Display>(&self, t: T) -> Colored<T>;
    fn stop_op<T: Display>(&self, t: T) -> Colored<T>;
    fn control_flow_op<T: Display>(&self, t: T) -> Colored<T>;
    fn data_op<T: Display>(&self, t: T) -> Colored<T>;
    fn comparison_op<T: Display>(&self, t: T) -> Colored<T>;
    fn invalid_op<T: Display>(&self, t: T) -> Colored<T>;
    fn platform_op<T: Display>(&self, t: T) -> Colored<T>;

    fn register<T: Display>(&self, t: T) -> Colored<T>;
    fn program_counter<T: Display>(&self, t: T) -> Colored<T>;
    fn number<T: Display>(&self, t: T) -> Colored<T>;
    fn zero<T: Display>(&self, t: T) -> Colored<T>;
    fn one<T: Display>(&self, t: T) -> Colored<T>;
    fn minus_one<T: Display>(&self, t: T) -> Colored<T>;
    fn address<T: Display>(&self, t: T) -> Colored<T>;
    fn symbol<T: Display>(&self, t: T) -> Colored<T>;
    fn function<T: Display>(&self, t: T) -> Colored<T>;
}

impl YaxColors for ColorSettings {
    fn arithmetic_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.arithmetic)
    }
    fn stack_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.stack)
    }
    fn nop_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.nop)
    }
    fn stop_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.stop)
    }
    fn control_flow_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.control)
    }
    fn data_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.data)
    }
    fn comparison_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.comparison)
    }
    fn invalid_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.invalid)
    }
    fn platform_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.platform)
    }

    fn register<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.register)
    }
    fn program_counter<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.program_counter)
    }
    fn number<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.number)
    }
    fn zero<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.zero)
    }
    fn one<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.one)
    }
    fn minus_one<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.minus_one)
    }
    fn address<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.address)
    }
    fn symbol<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.symbol)
    }
    fn function<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Color(t, self.function)
    }
}

impl <'a> YaxColors for Option<&'a ColorSettings> {
    fn arithmetic_op<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.arithmetic_op(t) }
            None => { Colored::Just(t) }
        }
    }
    fn stack_op<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.stack_op(t) }
            None => { Colored::Just(t) }
        }
    }
    fn nop_op<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.nop_op(t) }
            None => { Colored::Just(t) }
        }
    }
    fn stop_op<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.stop_op(t) }
            None => { Colored::Just(t) }
        }
    }
    fn control_flow_op<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.control_flow_op(t) }
            None => { Colored::Just(t) }
        }
    }
    fn data_op<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.data_op(t) }
            None => { Colored::Just(t) }
        }
    }
    fn comparison_op<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.comparison_op(t) }
            None => { Colored::Just(t) }
        }
    }
    fn invalid_op<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.invalid_op(t) }
            None => { Colored::Just(t) }
        }
    }
    fn platform_op<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.platform_op(t) }
            None => { Colored::Just(t) }
        }
    }

    fn register<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.register(t) }
            None => { Colored::Just(t) }
        }
    }
    fn program_counter<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.program_counter(t) }
            None => { Colored::Just(t) }
        }
    }
    fn number<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.number(t) }
            None => { Colored::Just(t) }
        }
    }
    fn zero<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.zero(t) }
            None => { Colored::Just(t) }
        }
    }
    fn one<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.one(t) }
            None => { Colored::Just(t) }
        }
    }
    fn minus_one<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.minus_one(t) }
            None => { Colored::Just(t) }
        }
    }
    fn address<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.address(t) }
            None => { Colored::Just(t) }
        }
    }
    fn symbol<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.symbol(t) }
            None => { Colored::Just(t) }
        }
    }
    fn function<T: Display>(&self, t: T) -> Colored<T> {
        match self {
            Some(colors) => { colors.function(t) }
            None => { Colored::Just(t) }
        }
    }
}

pub enum Colored<T: Display> {
    Color(T, color::Fg<&'static color::Color>),
    Just(T)
}

impl <T: Display> Display for Colored<T> {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        match self {
            Colored::Color(t, color) => {
                write!(fmt, "{}{}{}", color, t, color::Fg(color::Reset))
            },
            Colored::Just(t) => {
                write!(fmt, "{}", t)
            }
        }
    }
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
