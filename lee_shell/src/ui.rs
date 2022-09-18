use crate::history::History;
use std::collections::{HashMap, VecDeque};
use std::env::{current_dir, set_current_dir};
use std::io::{stderr, stdin, stdout, Read, Write};
use std::path::PathBuf;

pub struct UI {}

impl UI {
    pub fn new() -> UI {
        UI {}
    }
    pub fn get_input(&self) -> String {
        print!("{}:", current_dir().unwrap().to_str().unwrap());
        stdout().flush().expect("Flush failed.");
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Can not read.");
        s
    }
}
