use core::fmt::{self, Write};

use crate::sbi::putchar;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            putchar(c as usize);
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::_print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::console::_print("\n")
    };
    ($fmt: literal $(, $($arg: tt)+)?) => {{
        $crate::console::_print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }};
}