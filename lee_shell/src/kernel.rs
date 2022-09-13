use crate::ui::{UI};
use crate::default_env_vars::{DEFAULT_ENV_VARS};
use std::collections::HashMap;
use std::env;
use std::fmt::format;
use regex::Regex;
use std::path::{PathBuf};
use std::env::{current_dir};


pub struct Kernel {
	ui: UI,
	pwd: PathBuf,
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
			pwd: current_dir().unwrap(),
		}
	}
	pub fn work(&self) {
		while (true) {
			let script = self.ui.get_input(&self.pwd);
			let script = self.preprocess_script(&script);
			dbg!(script);
		}
	}
}