//! A simple `input!` macro with minimal functionality.
//!
//! ```no_run
//! #[macro_use]
//! extern crate input as _;
//!
//! fn main() {
//!     input! {
//!         a: [u64],
//!     }
//! }
//! ```

use std::{
    fmt,
    io::{self, Read},
    str::{FromStr, SplitAsciiWhitespace},
};

#[macro_export]
macro_rules! input {
    ($($tt:tt)*) => {
        let mut __scanner = $crate::Scanner::new().unwrap();
        $crate::input_inner!(@scanner(__scanner), @tts($($tt)*));
        ::std::mem::drop(__scanner);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! input_inner {
    (@scanner($scanner:ident), @tts()) => {};
    (@scanner($scanner:ident), @tts(mut $single_tt_pat:tt : $readable:tt)) => {
        let mut $single_tt_pat = $crate::read!(from $scanner { $readable });
    };
    (@scanner($scanner:ident), @tts($single_tt_pat:tt : $readable:tt)) => {
        let $single_tt_pat = $crate::read!(from $scanner { $readable });
    };
    (@scanner($scanner:ident), @tts(mut $single_tt_pat:tt : $readable:tt, $($rest:tt)*)) => {
        $crate::input_inner!(@scanner($scanner), @tts(mut $single_tt_pat: $readable));
        $crate::input_inner!(@scanner($scanner), @tts($($rest)*));
    };
    (@scanner($scanner:ident), @tts($single_tt_pat:tt : $readable:tt, $($rest:tt)*)) => {
        $crate::input_inner!(@scanner($scanner), @tts($single_tt_pat: $readable));
        $crate::input_inner!(@scanner($scanner), @tts($($rest)*));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! read {
    (from $scanner:ident { [$tt:tt] }) => {
        $crate::read!(from $scanner { [$tt; $crate::read!(from $scanner { usize })] })
    };
    (from $scanner:ident  { [$tt:tt; $n:expr] }) => {
        (0..$n).map(|_| $crate::read!(from $scanner { $tt })).collect::<Vec<_>>()
    };
    (from $scanner:ident { ($($tt:tt),+) }) => {
        ($($crate::read!(from $scanner { $tt })),*)
    };
    (from $scanner:ident { $ty:ty }) => {
        $scanner.parse::<$ty>()
    };
}

#[doc(hidden)]
pub struct Scanner {
    words: SplitAsciiWhitespace<'static>,
}

impl Scanner {
    pub fn new() -> io::Result<Self> {
        let mut buf = String::with_capacity(1024);
        io::stdin().read_to_string(&mut buf)?;
        let words = Box::leak(buf.into_boxed_str()).split_ascii_whitespace();
        Ok(Self { words })
    }

    /// Parses the next word.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// - reached the end of input.
    /// - the word is not successfully parsed.
    pub fn parse<T>(&mut self) -> T
    where
        T: FromStr,
        T::Err: fmt::Display,
    {
        let word = self.words.next().expect("reached the end of input");
        word.parse()
            .unwrap_or_else(|e| panic!("could not parse {:?}: {}", word, e))
    }
}
