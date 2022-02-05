use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::{clear, color, style};

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let answer = vec!['q', 'u', 'e', 'r', 'y'];
    let mut letters = vec![];

    write!(stdout, "wordl-tui\r\nctrl+c to exit\r\n",).unwrap();
    stdout.flush().unwrap();

    write!(stdout, "\r");
    let last_guess = vec!['d', 'r', 'e', 's', 's'];
    for letter in last_guess {
        if letter == 'r' {
            write!(
                stdout,
                "{}{}{}",
                color::Fg(color::Yellow),
                style::Underline,
                letter
            );
        } else if letter == 'e' {
            write!(
                stdout,
                "{}{}{}",
                color::Fg(color::Green),
                style::Underline,
                letter
            );
        } else {
            write!(stdout, "{}{}", style::Reset, letter);
        }
    }
    write!(stdout, "{}\r\n", style::Reset);
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Ctrl('c')) => break,
            Event::Key(Key::Backspace) => {
                letters.pop();
                write!(stdout, "{}", format_input(&letters));
            }
            Event::Key(Key::Char(key)) => {
                if letters.len() <= 5 {
                    letters.push(key);
                    write!(stdout, "{}", format_input(&letters));
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}

fn format_input(letters: &Vec<char>) -> String {
    format!(
        "\r{}{}",
        clear::CurrentLine,
        letters.clone().iter().collect::<String>()
    )
}
