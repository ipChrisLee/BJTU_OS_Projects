use crate::ui::{UI};
use std::env;
use std::fmt::format;
use regex::Regex;
use std::path::{PathBuf};
use std::env::{current_dir,var,set_current_dir};
use crate::runner::run;
use crate::lsh_parser::{parse_string,Command};


pub struct Kernel {
	ui: UI,
}

impl Kernel {
	/// Expand env variable and erase '\n'.
	fn preprocess_script(&self, script: &String) -> String {
		let mut script = script.clone();
		for (name,val) in env::vars(){
			let from = format!("${{{}}}",name);
			script=script.replace(from.as_str(),val.as_str());
		}
		script.trim().to_string()
	}
	pub fn new() -> Kernel {
		Kernel {
			ui: UI::new(),
		}
	}
	pub fn work(&self) {
		loop {
			let script = self.ui.get_input();
			let script = self.preprocess_script(&script);
			let cmds=parse_string(script.as_str());
			run(cmds);
		}
	}
}