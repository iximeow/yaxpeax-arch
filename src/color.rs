use core::fmt::{self, Display, Formatter};
#[cfg(feature="colors")]
use crossterm::style;

#[cfg(feature="colors")]
pub enum Colored<T: Display> {
    Color(T, style::Color),
    Just(T)
}

#[cfg(feature="colors")]
impl <T: Display> Display for Colored<T> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Colored::Color(t, before) => {
                write!(fmt, "{}", style::style(t).with(*before))
            },
            Colored::Just(t) => {
                write!(fmt, "{}", t)
            }
        }
    }
}

#[cfg(not(feature="colors"))]
pub enum Colored<T: Display> {
    Just(T)
}

#[cfg(not(feature="colors"))]
impl <T: Display> Display for Colored<T> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Colored::Just(t) => {
                write!(fmt, "{}", t)
            }
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
    fn misc_op<T: Display>(&self, t: T) -> Colored<T>;

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

pub struct NoColors;

impl YaxColors for NoColors {
    fn arithmetic_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn stack_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn nop_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn stop_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn control_flow_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn data_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn comparison_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn invalid_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn platform_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn misc_op<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn register<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn program_counter<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn number<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn zero<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn one<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn minus_one<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn address<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn symbol<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
    fn function<T: Display>(&self, t: T) -> Colored<T> {
        Colored::Just(t)
    }
}

pub trait Colorize<T: fmt::Write, Y: YaxColors + ?Sized> {
    fn colorize(&self, colors: &Y, out: &mut T) -> fmt::Result;
}

#[cfg(feature="colors")]
pub use termion_color::ColorSettings;

#[cfg(feature="colors")]
mod termion_color {
    use core::fmt::Display;

    use crossterm::style;

    use serde::Serialize;

    use crate::color::{Colored, YaxColors};

    #[cfg(feature="use-serde")]
    impl Serialize for ColorSettings {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use serde::ser::SerializeStruct;
            let s = serializer.serialize_struct("ColorSettings", 0)?;
            s.end()
        }
    }

    pub struct ColorSettings {
        arithmetic: style::Color,
        stack: style::Color,
        nop: style::Color,
        stop: style::Color,
        control: style::Color,
        data: style::Color,
        comparison: style::Color,
        invalid: style::Color,
        platform: style::Color,
        misc: style::Color,

        register: style::Color,
        program_counter: style::Color,

        number: style::Color,
        zero: style::Color,
        one: style::Color,
        minus_one: style::Color,

        function: style::Color,
        symbol: style::Color,
        address: style::Color,
    }

    impl Default for ColorSettings {
        fn default() -> ColorSettings {
            ColorSettings {
                arithmetic: style::Color::Yellow,
                stack: style::Color::DarkMagenta,
                nop: style::Color::DarkBlue,
                stop: style::Color::Red,
                control: style::Color::DarkGreen,
                data: style::Color::Magenta,
                comparison: style::Color::DarkYellow,
                invalid: style::Color::DarkRed,
                platform: style::Color::DarkCyan,
                misc: style::Color::Cyan,

                register: style::Color::DarkCyan,
                program_counter: style::Color::DarkRed,

                number: style::Color::White,
                zero: style::Color::White,
                one: style::Color::White,
                minus_one: style::Color::White,

                function: style::Color::Green,
                symbol: style::Color::Green,
                address: style::Color::DarkGreen,
            }
        }
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
        fn misc_op<T: Display>(&self, t: T) -> Colored<T> {
            Colored::Color(t, self.misc)
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
        fn misc_op<T: Display>(&self, t: T) -> Colored<T> {
            match self {
                Some(colors) => { colors.misc_op(t) }
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
}

/*
 * can this be a derivable trait or something?
 */
/*
impl <T: Colorize> Display for T {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        self.colorize(None, fmt)
    }
}
*/

/*
 * and make this auto-derive from a ShowContextual impl?
 */
/*
impl <T, U> Colorize for T where T: ShowContextual<Ctx=U> {
    fn colorize(&self, colors: Option<&ColorSettings>, fmt: &mut Formatter) -> fmt::Result {
        self.contextualize(colors, None, fmt)
    }
}
*/
