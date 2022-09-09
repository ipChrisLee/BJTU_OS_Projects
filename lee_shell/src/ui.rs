use std::collections::HashMap;
use std::io::{stdin, stdout, stderr, Write, Read};

pub struct UI {}

impl UI {
	pub fn new() -> UI {
		UI {}
	}
	pub fn get_input(&self, pwd: &str) -> String {
		print!("{}:", pwd);
		stdout().flush().expect("Flush failed.");
		let mut s = String::new();
		stdin().read_line(&mut s).expect("Can not read.");
		s
	}
}