extern crate ansi_term;

use ansi_term::{ANSIGenericString, Style};
use ansi_term::Colour::Red;

pub fn show(text: String) {
    println!("{} {}", normal_prefix(), text);
}

pub fn show_err(text: String) {
    println!("{} {}", error_prefix(), text);
}

fn normal_prefix<'a>() -> ANSIGenericString<'a, str> {
    Style::new().bold().paint("chlog:")
}

fn error_prefix<'a>() -> ANSIGenericString<'a, str> {
    Style::new().bold().fg(Red).paint("chlog:")
}
