use std::ops::{Deref, DerefMut};
// fyi: I tried to first ad-hoc everything, then I tried to clean things up, then I decided to fuckall speedrun everything.
//      Curses drove me to my mental limit

use pancurses::{A_STANDOUT, Attribute, Window};

use crate::{args::Arguments, terminal::Terminal};

pub struct Prompt {
    window: Window,
    pub cursor: u8,
    pub buffer: String,
}

const PADDING: i32 = 2;
const BORDER: i32 = 1;

const WIDTH: i32 = 70;

impl Prompt {
    pub fn create(screen: &Terminal, context: &Arguments) -> Self {
        let height = Self::calculate_height(context);
        let window = screen
            .subwin(
                height,
                WIDTH,
                (screen.height - height) / 2,
                (screen.width - WIDTH) / 2,
            )
            .unwrap();
        let mut prompt = Self {
            window,
            cursor: 0,
            buffer: String::new(),
        };

        prompt.redraw(screen, context);

        prompt
    }

    fn calculate_height(context: &Arguments) -> i32 {
        let mut height = PADDING + 3;

        if let Some(ref annotation) = context.annotation {
            height += annotation.split('\n').count() as i32 + 2;
        }

        height
    }

    pub fn redraw(&mut self, screen: &Terminal, context: &Arguments) {
        let width = screen.width.min(context.max_width + PADDING * 2);
        if width < WIDTH {
            self.resize(2, 13);
            self.mvwin((screen.height - 2) / 2, (screen.width - 13) / 2);

            self.mv(0, 0);
            self.printw("minimum size:");
            self.mv(1, 0);
            self.printw("    30x10    ");

            self.refresh();
            return;
        }

        let mut cursor = (0, 0);

        self.draw_box(0, 0);

        self.mv(1, 2);
        if let Some(ref annotation) = context.annotation {
            for line in annotation.split('\n') {
                self.printw(line);
                self.mv(self.get_cur_y() + 1, 2);
            }
            self.mv(self.get_cur_y() + 2, 2);
        }

        self.printw(format!("{}: ", context.prompt));

        let width = WIDTH - PADDING * 2 - context.prompt.len() as i32 - 2;
        if self.cursor == 0 {
            cursor = self.get_cur_yx();
            cursor.1 += i32::min(self.buffer.len() as i32, width - 1);
        }

        self.hline('*', i32::min(self.buffer.len() as i32, width));
        self.mv(
            self.get_cur_y(),
            PADDING + self.buffer.len() as i32 + context.prompt.len() as i32 + 2,
        );
        self.hline('_', width - self.buffer.len() as i32);

        self.mv(self.get_cur_y() + 2, 12);

        if self.cursor == 1 {
            cursor = self.get_cur_yx();
            cursor.1 += 1;
            self.attron(A_STANDOUT);
        }

        self.printw("<OK>");
        self.attroff(A_STANDOUT);

        self.mv(self.get_cur_y(), 51);
        if self.cursor == 2 {
            cursor = self.get_cur_yx();
            cursor.1 += 1;
            self.attron(A_STANDOUT);
        }

        self.printw("<Cancel>");
        self.attroff(A_STANDOUT);

        let (y, x) = cursor;
        self.mv(y, x);

        self.refresh();
    }
}

impl Deref for Prompt {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for Prompt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}
