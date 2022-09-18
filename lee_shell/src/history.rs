use crate::config::HISTORY_NUMBER;
use std::collections::vec_deque::Iter;
use std::iter::Rev;
use std::{collections::VecDeque, iter::Enumerate};

pub struct History {
    history: VecDeque<String>,
}

impl History {
    pub fn new() -> History {
        assert!(HISTORY_NUMBER > 2);
        History {
            history: VecDeque::new(),
        }
    }
    pub fn add_history(&mut self, s: String) {
        self.history.push_front(s);
        while self.history.len() > HISTORY_NUMBER {
            self.history.pop_back();
        }
    }
    pub fn all_history(&self) -> Rev<Enumerate<Iter<String>>> {
        self.history.iter().enumerate().rev()
    }
    pub fn get_n_history(&self, n: usize) -> Option<&String> {
        self.history.get(n)
    }
}
