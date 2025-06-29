use std::{
    ops::Deref,
    os::fd::{AsFd, AsRawFd},
};

use libc::{FILE, PT_NULL, STDERR_FILENO, fopen};
use pancurses::{Window, endwin, initscr, newterm, newwin, set_term};

pub struct Terminal {
    window: Window,

    pub width: i32,
    pub height: i32,
}

unsafe extern "C" {
    static stderr: *mut FILE;
    static stdin: *mut FILE;
}

impl Terminal {
    pub fn init() -> Self {
        newterm(None, unsafe { stderr }, unsafe { stdin });
        let window = newwin(0, 0, 0, 0);

        let (height, width) = window.get_max_yx();

        Self {
            window: window,
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }
}

impl Deref for Terminal {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.window.keypad(false);

        endwin();
    }
}
