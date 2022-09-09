use crate::ui::{UI};
use crate::default_env_vars::{DEFAULT_ENV_VARS};
use std::collections::HashMap;
use std::fmt::format;
use regex::Regex;

pub struct Kernel {
	ui: UI,
	env_vars: HashMap<String, String>,
}


impl Kernel {
	/// Expand env variable and erase '\n'.
	fn preprocess_script(&self, script: &String) -> String {
		let mut script = script.clone();
		for (name, val) in &self.env_vars {
			let from = format!("{}{}", "$", name);
			script = script.replace(from.as_str(), val.as_str());
		}
		script.trim().to_string()
	}
	fn var_get_val(&self, name: &str) -> String {
		match self.env_vars.get(name) {
			None => String::from(""),
			Some(s) => s.clone()
		}
	}
	pub fn new() -> Kernel {
		Kernel {
			ui: UI::new(),
			env_vars: HashMap::from(DEFAULT_ENV_VARS.map(|(k, v)| (k.to_string(), v.to_string()))),
		}
	}
	pub fn work(&self) {
		while (true) {
			let script = self.ui.get_input(self.var_get_val("HOME").as_str());
			let script = self.preprocess_script(&script);
			dbg!(script);
		}
	}
}