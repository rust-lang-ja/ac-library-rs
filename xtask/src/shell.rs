use std::{
    fmt,
    io::{self, Write as _},
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor as _};

pub struct Shell {
    stderr: StandardStream,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            stderr: StandardStream::stderr(if atty::is(atty::Stream::Stderr) {
                ColorChoice::Auto
            } else {
                ColorChoice::Never
            }),
        }
    }

    pub(crate) fn err(&mut self) -> &mut StandardStream {
        &mut self.stderr
    }

    pub(crate) fn status(
        &mut self,
        status: impl fmt::Display,
        message: impl fmt::Display,
    ) -> io::Result<()> {
        self.print(status, message, Color::Green, true)
    }

    pub fn error(&mut self, message: impl fmt::Display) -> io::Result<()> {
        self.print("error", message, Color::Red, false)
    }

    fn print(
        &mut self,
        status: impl fmt::Display,
        message: impl fmt::Display,
        color: Color,
        justified: bool,
    ) -> io::Result<()> {
        self.stderr
            .set_color(ColorSpec::new().set_bold(true).set_fg(Some(color)))?;
        if justified {
            write!(self.stderr, "{:>12}", status)?;
        } else {
            write!(self.stderr, "{}", status)?;
            self.stderr.set_color(ColorSpec::new().set_bold(true))?;
            write!(self.stderr, ":")?;
        }
        self.stderr.reset()?;
        writeln!(self.stderr, " {}", message)
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}
