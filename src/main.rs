mod lib;

use lib::io::Writer;
use lib::state::{Game, Letter};
use std::io::stdin;
use termion::event::{Event, Key};
use termion::input::TermRead;

fn main() {
    let stdin = stdin();
    let mut game = Game::new("query");
    let mut write = Writer::new();

    write.line("wordl-tui");
    write.line("(ctrl+c to exit)");
    write.newline();

    for (i, letter) in ['d', 'r', 'e', 's', 's'].iter().enumerate() {
        match game.check_guess(letter, i) {
            Letter::Green(_) => write.green_guess(letter),
            Letter::Yellow(_) => write.yellow_guess(letter),
            _ => write.guess(letter),
        }
    }

    write.newline();
    write.flush();

    let mut input = vec![];
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Ctrl('c')) => break,
            Event::Key(Key::Backspace) => {
                input.pop();
                write.backspace();
            }
            Event::Key(Key::Char(key)) => {
                let position = input.len();

                // Enter key
                if key == '\n' {
                    if position == 5 {
                        write.input(&key);
                        write.flush();
                        break;
                    }
                    continue;
                }

                if position < 5 {
                    input.push(key);
                    match game.check_input(&key, position) {
                        Letter::Green(_) => write.green_input(&key),
                        Letter::Yellow(_) => write.yellow_input(&key),
                        Letter::Red(_) => write.red_input(&key),
                        _ => write.input(&key),
                    }
                }
            }
            _ => {}
        }
        write.flush();
    }
}
