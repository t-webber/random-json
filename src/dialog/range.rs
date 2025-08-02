use std::{
    fmt,
    io::{BufRead as _, Lines, StdinLock, Write as _, stdin, stdout},
    ops::Range,
};

use crate::errors::Res;

macro_rules! print_flush {
    ($($arg:tt)*) => {{
        print!($($arg)*);
        stdout().flush()?;
    }};
}

enum TermCodes {
    PromptIcon,
    OkIcon,
    ErrorIcon,
    MoveUp,
    MoveDown,
}

impl fmt::Display for TermCodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PromptIcon => "\x1b[33m?\x1b[0m",
            Self::OkIcon => "\x1b[32m✔\x1b[0m",
            Self::ErrorIcon => "\x1b[31m✘\x1b[0m",
            Self::MoveUp => "\x1B[1A",
            Self::MoveDown => "\x1B[1B",
        }
        .fmt(f)
    }
}

fn read_int(lines: &mut Lines<StdinLock<'static>>) -> Option<usize> {
    lines.next()?.ok()?.trim().parse().ok()
}

pub fn get_range() -> Res<Range<usize>> {
    use TermCodes::*;

    let stdin = stdin();
    let mut lines = stdin.lock().lines();

    let mut read_num1;
    let num1 = loop {
        print_flush!("{PromptIcon} Enter lower bound: ");
        read_num1 = read_int(&mut lines);
        if let Some(num1) = read_num1 {
            print_flush!("{MoveUp}{OkIcon}{MoveDown}\r");
            break num1;
        } else {
            print_flush!("{MoveUp}{ErrorIcon}{MoveDown}\r");
        }
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
        } else {
            print_flush!("{MoveUp}{ErrorIcon}{MoveDown}\r");
        }
    };

    Ok(num1..num2)
}
