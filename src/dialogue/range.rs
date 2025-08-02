//! Module for handling user input to get a range of integers interactively.

use core::fmt;
use core::ops::Range;
use std::io::{BufRead as _, Lines, StdinLock, Write as _, stdin, stdout};

use crate::errors::Res;

/// Print a formatted string to the standard output and flush it immediately,
/// without needing a new line.
macro_rules! print_flush {
    ($($arg:tt)*) => {{
        print!($($arg)*);
        stdout().flush()?;
    }};
}

/// Terminal sequences to display icons and control cursor movement.
enum TermCodes {
    /// Display an icon when the user input is invalid
    ErrorIcon,
    /// Move the cursor down one line in the terminal
    MoveDown,
    /// Move the cursor up one line in the terminal
    MoveUp,
    /// Display an icon when the user input is valid
    OkIcon,
    /// Display an icon when prompting the user
    PromptIcon,
}

#[expect(clippy::min_ident_chars, reason = "fix on the way, cf. #15275")]
impl fmt::Display for TermCodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PromptIcon => "\x1b[33m?\x1b[0m",
            Self::OkIcon => "\x1b[32m\u{2714}\x1b[0m",
            Self::ErrorIcon => "\x1b[31m\u{2718}\x1b[0m",
            Self::MoveUp => "\x1B[1A",
            Self::MoveDown => "\x1B[1B",
        }
        .fmt(f)
    }
}

/// Try to read an integer from the user input
fn read_int(lines: &mut Lines<StdinLock<'static>>) -> Option<usize> {
    lines.next()?.ok()?.trim().parse().ok()
}

/// Get a range of integers from an interactive user input.
pub fn get_range() -> Res<Range<usize>> {
    use TermCodes::{ErrorIcon, MoveDown, MoveUp, OkIcon, PromptIcon};

    let stdin = stdin();
    let mut lines = stdin.lock().lines();

    let mut read_num1;
    let num1 = loop {
        print_flush!("{PromptIcon} Enter lower bound: ");
        read_num1 = read_int(&mut lines);
        if let Some(num1) = read_num1 {
            print_flush!("{MoveUp}{OkIcon}{MoveDown}\r");
            break num1;
        }
        print_flush!("{MoveUp}{ErrorIcon}{MoveDown}\r");
    };

    let mut read_num2;
    let num2 = loop {
        print_flush!("{PromptIcon} Enter upper bound: ");
        read_num2 = read_int(&mut lines);
        if let Some(num2) = read_num2
            && num1 < num2
        {
            print_flush!("{MoveUp}{OkIcon}{MoveDown}\r");
            break num2;
        }
        print_flush!("{MoveUp}{ErrorIcon}{MoveDown}\r");
    };

    Ok(num1..num2)
}
