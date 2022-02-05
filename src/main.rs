mod lib;

use lib::io::Writer;
use std::io::stdin;
use termion::event::{Event, Key};
use termion::input::TermRead;

fn main() {
    let stdin = stdin();
    let answer = vec!['q', 'u', 'e', 'r', 'y'];
    let mut letters = vec![];
    let mut write = Writer::new();

    write.line("wordl-tui");
    write.line("(ctrl+c to exit)");
    write.newline();

    let last_guess = vec!['d', 'r', 'e', 's', 's'];
    print_guess(&mut write, &last_guess, &answer);
    write.flush();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Ctrl('c')) => break,
            Event::Key(Key::Backspace) => {
                letters.pop();
                write.backspace();
            }
            Event::Key(Key::Char(key)) => {
                // Enter key
                if key == '\n' {
                    if letters.len() == 5 {
                        write.input(&key);
                        write.flush();
                        break;
                    }
                    continue;
                }

                if letters.len() < 5 {
                    letters.push(key);
                    if key == 'r' {
                        write.yellow_input(&key);
                    } else if key == 'e' {
                        write.green_input(&key);
                    } else {
                        write.input(&key);
                    }
                }
            }
            _ => {}
        }
        write.flush();
    }
}

fn print_guess(write: &mut Writer, guess: &Vec<char>, answer: &Vec<char>) {
    for (index, letter) in guess.iter().enumerate() {
        if letter == &answer[index] {
            write.green_guess(letter);
        } else if answer.contains(letter) {
            write.yellow_guess(letter);
        } else {
            write.guess(letter);
        }
    }

    write.newline();
    write.flush();
}
