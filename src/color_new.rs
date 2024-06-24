#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Color {
    Black,
    DarkGrey,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    White,
    Grey,
}

pub trait YaxColors {
    fn arithmetic_op(&self) -> Color;
    fn stack_op(&self) -> Color;
    fn nop_op(&self) -> Color;
    fn stop_op(&self) -> Color;
    fn control_flow_op(&self) -> Color;
    fn data_op(&self) -> Color;
    fn comparison_op(&self) -> Color;
    fn invalid_op(&self) -> Color;
    fn platform_op(&self) -> Color;
    fn misc_op(&self) -> Color;

    fn register(&self) -> Color;
    fn program_counter(&self) -> Color;
    fn number(&self) -> Color;
    fn zero(&self) -> Color;
    fn one(&self) -> Color;
    fn minus_one(&self) -> Color;
    fn address(&self) -> Color;
    fn symbol(&self) -> Color;
    fn function(&self) -> Color;
}

/// support for colorizing text with ANSI control sequences.
///
/// the most useful item in this module is [`AnsiDisplaySink`], which interprets span entry and
/// exit as points at which ANSI sequences may need to be written into the output it wraps - that
/// output may be any type implementing [`DisplaySink`], including [`FmtSink`] to adapt any
/// implementer of `fmt::Write` such as standard out.
///
/// ## example
///
/// to write colored text to standard out:
///
/// ```
/// # #[cfg(feature="alloc")]
/// # {
/// # extern crate alloc;
/// # use alloc::string::String;
/// use yaxpeax_arch::color_new::DefaultColors;
/// use yaxpeax_arch::color_new::ansi::AnsiDisplaySink;
/// use yaxpeax_arch::display::FmtSink;
///
/// let mut s = String::new();
/// let mut s_sink = FmtSink::new(&mut s);
///
/// let mut writer = AnsiDisplaySink::new(&mut s_sink, DefaultColors);
///
/// // this might be a yaxpeax crate's `display_into`, or other library implementation code
/// mod fake_yaxpeax_crate {
///     use yaxpeax_arch::display::DisplaySink;
///
///     pub fn format_memory_operand<T: DisplaySink>(out: &mut T) -> core::fmt::Result {
///         out.span_start_immediate();
///         out.write_prefixed_u8(0x80)?;
///         out.span_end_immediate();
///         out.write_fixed_size("(")?;
///         out.span_start_register();
///         out.write_fixed_size("rbp")?;
///         out.span_end_register();
///         out.write_fixed_size(")")?;
///         Ok(())
///     }
/// }
///
/// // this might be how a user uses `AnsiDisplaySink`, which will write ANSI-ful text to `s` and
/// // print it.
///
/// fake_yaxpeax_crate::format_memory_operand(&mut writer).expect("write succeeds");
///
/// println!("{}", s);
/// # }
/// ```
pub mod ansi {
    use crate::color_new::Color;

    // color sequences as described by ECMA-48 and, apparently, `man 4 console_codes`
    /// translate [`yaxpeax_arch::color_new::Color`] to an ANSI control code that changes the
    /// foreground color to match.
    #[allow(dead_code)] // allowing this to be dead code because if colors are enabled and alloc is not, there will not be an AnsiDisplaySink, which is the sole user of this function.
    fn color2ansi(color: Color) -> &'static str {
        // for most of these, in 256 color space the darker color can be picked by the same color
        // index as the brighter form (from the 8 color command set). dark grey is an outlier,
        // where 38;5;0 and 30 both are black. there is no "grey" in the shorter command set to
        // map to. but it turns out that 38;5;m is exactly the darker grey to use.
        match color {
            Color::Black => "\x1b[30m",
            Color::DarkGrey => "\x1b[38;5;8m",
            Color::Red => "\x1b[31m",
            Color::DarkRed => "\x1b[38;5;1m",
            Color::Green => "\x1b[32m",
            Color::DarkGreen => "\x1b[38;5;2m",
            Color::Yellow => "\x1b[33m",
            Color::DarkYellow => "\x1b[38;5;3m",
            Color::Blue => "\x1b[34m",
            Color::DarkBlue => "\x1b[38;5;4m",
            Color::Magenta => "\x1b[35m",
            Color::DarkMagenta => "\x1b[38;5;5m",
            Color::Cyan => "\x1b[36m",
            Color::DarkCyan => "\x1b[38;5;6m",
            Color::White => "\x1b[37m",
            Color::Grey => "\x1b[38;5;7m",
        }
    }

    // could reasonably be always present, but only used if feature="alloc"
    #[cfg(feature="alloc")]
    const DEFAULT_FG: &'static str = "\x1b[39m";

    #[cfg(feature="alloc")]
    mod ansi_display_sink {
        use crate::color_new::{Color, YaxColors};
        use crate::display::DisplaySink;

        /// adapter to insert ANSI color command sequences in formatted text to style printed
        /// instructions.
        ///
        /// this enables similar behavior as the deprecated [`yaxpeax_arch::colors::Colorize`] trait,
        /// for outputs that can process ANSI color commands.
        ///
        /// `AnsiDisplaySink` will silently ignore errors from writes to the underlying `T:
        /// DisplaySink`. when writing to a string or other growable buffer, errors are likely
        /// inseparable from `abort()`. when writing to stdout or stderr, write failures likely
        /// mean output is piped to a process which has closed the pipe but are otherwise harmless.
        /// `span_enter_*` and `span_exit_*` don't have error reporting mechanisms in their return
        /// type, so the only available error mechanism would be to also `abort()`.
        ///
        /// if this turns out to be a bad decision, it'll have to be rethought!
        pub struct AnsiDisplaySink<'sink, T: DisplaySink, Y: YaxColors> {
            out: &'sink mut T,
            span_stack: alloc::vec::Vec<Color>,
            colors: Y
        }

        impl<'sink, T: DisplaySink, Y: YaxColors> AnsiDisplaySink<'sink, T, Y> {
            pub fn new(out: &'sink mut T, colors: Y) -> Self {
                Self {
                    out,
                    span_stack: alloc::vec::Vec::new(),
                    colors,
                }
            }

            fn push_color(&mut self, color: Color) {
                self.span_stack.push(color);
                let _ = self.out.write_fixed_size(super::color2ansi(color));
            }

            fn restore_prev_color(&mut self) {
                let _ = self.span_stack.pop();
                if let Some(prev_color) = self.span_stack.last() {
                    let _ = self.out.write_fixed_size(super::color2ansi(*prev_color));
                } else {
                    let _ = self.out.write_fixed_size(super::DEFAULT_FG);
                };
            }
        }

        impl<'sink, T: DisplaySink, Y: YaxColors> core::fmt::Write for AnsiDisplaySink<'sink, T, Y> {
            fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
                self.out.write_str(s)
            }
            fn write_char(&mut self, c: char) -> Result<(), core::fmt::Error> {
                self.out.write_char(c)
            }
        }

        impl<'sink, T: DisplaySink, Y: YaxColors> DisplaySink for AnsiDisplaySink<'sink, T, Y> {
            fn span_start_immediate(&mut self) { self.push_color(self.colors.number()); }
            fn span_end_immediate(&mut self) { self.restore_prev_color() }

            fn span_start_register(&mut self) { self.push_color(self.colors.register()); }
            fn span_end_register(&mut self) { self.restore_prev_color() }

            // ah.. the right way, currently, to colorize opcodes would be to collect text while in the
            // opcode span, and request some kind of user-provided decoder ring to translate mnemonics
            // into the right color. that's very unfortunate. maybe there should be another span for
            // `opcode_kind(u8)` for impls to report what kind of opcode they'll be emitting..
            fn span_start_opcode(&mut self) { self.push_color(self.colors.misc_op()); }
            fn span_end_opcode(&mut self) { self.restore_prev_color() }

            fn span_start_program_counter(&mut self) { self.push_color(self.colors.program_counter()); }
            fn span_end_program_counter(&mut self) { self.restore_prev_color() }

            fn span_start_number(&mut self) { self.push_color(self.colors.number()); }
            fn span_end_number(&mut self) { self.restore_prev_color() }

            fn span_start_address(&mut self) { self.push_color(self.colors.address()); }
            fn span_end_address(&mut self) { self.restore_prev_color() }

            fn span_start_function_expr(&mut self) { self.push_color(self.colors.function()); }
            fn span_end_function_expr(&mut self) { self.restore_prev_color() }
        }
    }
    #[cfg(feature="alloc")]
    pub use ansi_display_sink::AnsiDisplaySink;
}

pub struct DefaultColors;

impl YaxColors for DefaultColors {
    fn arithmetic_op(&self) -> Color {
        Color::Yellow
    }
    fn stack_op(&self) -> Color {
        Color::DarkMagenta
    }
    fn nop_op(&self) -> Color {
        Color::DarkBlue
    }
    fn stop_op(&self) -> Color {
        Color::Red
    }
    fn control_flow_op(&self) -> Color {
        Color::DarkGreen
    }
    fn data_op(&self) -> Color {
        Color::Magenta
    }
    fn comparison_op(&self) -> Color {
        Color::DarkYellow
    }
    fn invalid_op(&self) -> Color {
        Color::DarkRed
    }
    fn misc_op(&self) -> Color {
        Color::Cyan
    }
    fn platform_op(&self) -> Color {
        Color::DarkCyan
    }

    fn register(&self) -> Color {
        Color::DarkCyan
    }
    fn program_counter(&self) -> Color {
        Color::DarkRed
    }
    fn number(&self) -> Color {
        Color::White
    }
    fn zero(&self) -> Color {
        Color::White
    }
    fn one(&self) -> Color {
        Color::White
    }
    fn minus_one(&self) -> Color {
        Color::White
    }
    fn address(&self) -> Color {
        Color::DarkGreen
    }
    fn symbol(&self) -> Color {
        Color::Green
    }
    fn function(&self) -> Color {
        Color::Green
    }
}
