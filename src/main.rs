use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::{clear, color, style};

type RawSTDOut = termion::input::MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>;

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let answer = vec!['q', 'u', 'e', 'r', 'y'];
    let mut letters = vec![];

    write!(stdout, "wordl-tui\r\nctrl+c to exit\r\n\n",).unwrap();
    stdout.flush().unwrap();

    let last_guess = vec!['d', 'r', 'e', 's', 's'];
    print_guess(&mut stdout, &last_guess, &answer);

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
                    if key == 'r' {
                        write!(
                            stdout,
                            "{}{}{}{}",
                            color::Fg(color::Yellow),
                            style::Underline,
                            key,
                            style::Reset
                        );
                    } else if key == 'e' {
                        write!(
                            stdout,
                            "{}{}{}{}",
                            color::Fg(color::Green),
                            style::Underline,
                            key,
                            style::Reset
                        );
                    } else {
                        write!(stdout, "{}", key);
                    }
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

fn print_guess(stdout: &mut RawSTDOut, guess: &Vec<char>, answer: &Vec<char>) {
    write!(stdout, "\r{}Guess #1:\r\n", style::Bold);

    for (index, letter) in guess.iter().enumerate() {
        if letter == &answer[index] {
            write!(
                stdout,
                "{}{}{}",
                color::Bg(color::Green),
                color::Fg(color::Black),
                letter
            );
        } else if answer.contains(letter) {
            write!(
                stdout,
                "{}{}{}",
                color::Bg(color::Yellow),
                color::Fg(color::Black),
                letter
            );
        } else {
            write!(stdout, "{}{}", style::Reset, letter);
        }
    }

    write!(stdout, "{}\r\n", style::Reset);
}
