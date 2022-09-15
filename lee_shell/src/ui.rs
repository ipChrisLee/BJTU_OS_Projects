use crate::config::HISTORY_NUMBER;
use std::collections::vec_deque::Iter;
use std::collections::{HashMap, VecDeque};
use std::env::{current_dir, set_current_dir};
use std::io::{stderr, stdin, stdout, Read, Write};
use std::path::PathBuf;

pub struct UI {
    history: VecDeque<String>,
}

impl UI {
    pub fn new() -> UI {
        assert!(HISTORY_NUMBER > 2);
        UI {
            history: VecDeque::new(),
        }
    }
    pub fn get_input(&mut self) -> String {
        print!("{}:", current_dir().unwrap().to_str().unwrap());
        stdout().flush().expect("Flush failed.");
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Can not read.");
        s
    }
    pub fn get_history(&self) -> Iter<String> {
        self.history.iter()
    }
}
