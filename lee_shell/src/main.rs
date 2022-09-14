extern crate pest;
#[macro_use]
extern crate pest_derive;

mod kernel;
mod ui;
mod default_env_vars;
mod lsh_parser;

use crate::kernel::{Kernel};

fn main() {
	let mut lee_shell = Kernel::new();
	lee_shell.work();
}
