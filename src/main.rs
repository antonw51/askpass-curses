mod args;
mod prompt;
mod terminal;

use std::{
    env::args,
    fmt::format,
    io::{Write, stdout},
    iter::repeat,
    process,
};

use anyhow::bail;
use pancurses::Input;

use crate::{args::Arguments, prompt::Prompt, terminal::Terminal};

struct State {
    cursor: u8,
}

fn main() {
    let mut args = std::env::args();
    let mut arguments = Arguments::parse_args(&mut args);

    let mut terminal = Terminal::init();
    let mut prompt = Prompt::create(&terminal, &arguments);

    prompt.redraw(&terminal, &arguments);

    prompt.keypad(true);

    loop {
        let Some(input) = prompt.getch() else { break };
        match input {
            Input::KeyEnter | Input::Character('\n') => {
                if prompt.cursor == 2 {
                    process::exit(1);
                }
                break;
            }
            Input::Character(c) => {
                if prompt.cursor == 0 {
                    prompt.buffer.push(c);
                }
            }
            Input::KeyBackspace => {
                if prompt.cursor == 0 {
                    prompt.buffer.pop();
                }
            }
            Input::KeyUp | Input::KeyLeft => {
                if prompt.cursor != 0 {
                    prompt.cursor -= 1;
                }
            }
            Input::KeyDown | Input::KeyRight => {
                if prompt.cursor < 2 {
                    prompt.cursor += 1;
                }
            }
            _ => (),
        }

        prompt.clear();
        prompt.redraw(&terminal, &arguments);
    }
    // i mean we dont need this if we dont drop the buffer and the term i guess  let buf = std::mem::take(&mut prompt.buffer);
    pancurses::endwin();
    let mut out = stdout(); // ye ik lemme try it anyways
    out.write_all(prompt.buffer.as_bytes()).expect("failed");
    out.flush().expect("failed2");
}
