extern crate ansi_term;

use ansi_term::{ANSIGenericString, Style};

pub fn show(text: String) {
    println!("{} {}", chlog_prefix(), text);
}

fn chlog_prefix<'a>() -> ANSIGenericString<'a, str> {
    Style::new().bold().paint("chlog:")
}
