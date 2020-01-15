use core::fmt::{self, Display, Formatter};

pub enum Colored<T: Display, Colorer: Display> {
    Color(T, Colorer, Colorer),
    Just(T)
}

impl <T: Display, Colorer: Display> Display for Colored<T, Colorer> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Colored::Color(t, before, after) => {
                write!(fmt, "{}{}{}", before, t, after)
            },
            Colored::Just(t) => {
                write!(fmt, "{}", t)
            }
        }
    }
}

pub trait YaxColors<Color: Display> {
    fn arithmetic_op<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn stack_op<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn nop_op<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn stop_op<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn control_flow_op<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn data_op<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn comparison_op<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn invalid_op<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn platform_op<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn misc_op<T: Display>(&self, t: T) -> Colored<T, Color>;

    fn register<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn program_counter<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn number<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn zero<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn one<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn minus_one<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn address<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn symbol<T: Display>(&self, t: T) -> Colored<T, Color>;
    fn function<T: Display>(&self, t: T) -> Colored<T, Color>;
}

pub struct NoColors;

impl YaxColors<&'static str> for NoColors {
    fn arithmetic_op<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn stack_op<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn nop_op<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn stop_op<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn control_flow_op<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn data_op<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn comparison_op<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn invalid_op<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn platform_op<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn misc_op<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn register<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn program_counter<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn number<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn zero<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn one<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn minus_one<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn address<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn symbol<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
    fn function<T: Display>(&self, t: T) -> Colored<T, &'static str> {
        Colored::Just(t)
    }
}

pub trait Colorize<T: fmt::Write, Color: Display, Y: YaxColors<Color> + ?Sized> {
    fn colorize(&self, colors: &Y, out: &mut T) -> fmt::Result;
}

#[cfg(feature="colors")]
pub use termion_color::ColorSettings;

#[cfg(feature="colors")]
mod termion_color {
    use core::fmt::Display;

    use termion::color;
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
        arithmetic: color::Fg<&'static dyn color::Color>,
        stack: color::Fg<&'static dyn color::Color>,
        nop: color::Fg<&'static dyn color::Color>,
        stop: color::Fg<&'static dyn color::Color>,
        control: color::Fg<&'static dyn color::Color>,
        data: color::Fg<&'static dyn color::Color>,
        comparison: color::Fg<&'static dyn color::Color>,
        invalid: color::Fg<&'static dyn color::Color>,
        platform: color::Fg<&'static dyn color::Color>,
        misc: color::Fg<&'static dyn color::Color>,

        register: color::Fg<&'static dyn color::Color>,
        program_counter: color::Fg<&'static dyn color::Color>,

        number: color::Fg<&'static dyn color::Color>,
        zero: color::Fg<&'static dyn color::Color>,
        one: color::Fg<&'static dyn color::Color>,
        minus_one: color::Fg<&'static dyn color::Color>,

        function: color::Fg<&'static dyn color::Color>,
        symbol: color::Fg<&'static dyn color::Color>,
        address: color::Fg<&'static dyn color::Color>,
    }

    #[cfg(feature="colorize")]
    impl Default for ColorSettings {
        fn default() -> ColorSettings {
            ColorSettings {
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

                register: color::Fg(&color::Cyan),
                program_counter: color::Fg(&color::Red),

                number: color::Fg(&color::White),
                zero: color::Fg(&color::White),
                one: color::Fg(&color::White),
                minus_one: color::Fg(&color::White),

                function: color::Fg(&color::LightGreen),
                symbol: color::Fg(&color::LightGreen),
                address: color::Fg(&color::Green),
            }
        }
    }

    impl YaxColors<color::Fg<&'static dyn color::Color>> for ColorSettings {
        fn arithmetic_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.arithmetic, color::Fg(&color::Reset))
        }
        fn stack_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.stack, color::Fg(&color::Reset))
        }
        fn nop_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.nop, color::Fg(&color::Reset))
        }
        fn stop_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.stop, color::Fg(&color::Reset))
        }
        fn control_flow_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.control, color::Fg(&color::Reset))
        }
        fn data_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.data, color::Fg(&color::Reset))
        }
        fn comparison_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.comparison, color::Fg(&color::Reset))
        }
        fn invalid_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.invalid, color::Fg(&color::Reset))
        }
        fn misc_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.misc, color::Fg(&color::Reset))
        }
        fn platform_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.platform, color::Fg(&color::Reset))
        }

        fn register<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.register, color::Fg(&color::Reset))
        }
        fn program_counter<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.program_counter, color::Fg(&color::Reset))
        }
        fn number<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.number, color::Fg(&color::Reset))
        }
        fn zero<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.zero, color::Fg(&color::Reset))
        }
        fn one<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.one, color::Fg(&color::Reset))
        }
        fn minus_one<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.minus_one, color::Fg(&color::Reset))
        }
        fn address<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.address, color::Fg(&color::Reset))
        }
        fn symbol<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.symbol, color::Fg(&color::Reset))
        }
        fn function<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            Colored::Color(t, self.function, color::Fg(&color::Reset))
        }
    }

    impl <'a> YaxColors<color::Fg<&'static dyn color::Color>>for Option<&'a ColorSettings> {
        fn arithmetic_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.arithmetic_op(t) }
                None => { Colored::Just(t) }
            }
        }
        fn stack_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.stack_op(t) }
                None => { Colored::Just(t) }
            }
        }
        fn nop_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.nop_op(t) }
                None => { Colored::Just(t) }
            }
        }
        fn stop_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.stop_op(t) }
                None => { Colored::Just(t) }
            }
        }
        fn control_flow_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.control_flow_op(t) }
                None => { Colored::Just(t) }
            }
        }
        fn data_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.data_op(t) }
                None => { Colored::Just(t) }
            }
        }
        fn comparison_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.comparison_op(t) }
                None => { Colored::Just(t) }
            }
        }
        fn invalid_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.invalid_op(t) }
                None => { Colored::Just(t) }
            }
        }
        fn misc_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.misc_op(t) }
                None => { Colored::Just(t) }
            }
        }
        fn platform_op<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.platform_op(t) }
                None => { Colored::Just(t) }
            }
        }

        fn register<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.register(t) }
                None => { Colored::Just(t) }
            }
        }
        fn program_counter<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.program_counter(t) }
                None => { Colored::Just(t) }
            }
        }
        fn number<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.number(t) }
                None => { Colored::Just(t) }
            }
        }
        fn zero<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.zero(t) }
                None => { Colored::Just(t) }
            }
        }
        fn one<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.one(t) }
                None => { Colored::Just(t) }
            }
        }
        fn minus_one<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.minus_one(t) }
                None => { Colored::Just(t) }
            }
        }
        fn address<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.address(t) }
                None => { Colored::Just(t) }
            }
        }
        fn symbol<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
            match self {
                Some(colors) => { colors.symbol(t) }
                None => { Colored::Just(t) }
            }
        }
        fn function<T: Display>(&self, t: T) -> Colored<T, color::Fg<&'static dyn color::Color>> {
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
