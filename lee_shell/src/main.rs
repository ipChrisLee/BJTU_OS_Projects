extern crate pest;
#[macro_use]
extern crate pest_derive;

mod kernel;
mod ui;
mod lsh_parser;
mod runner;

use crate::kernel::{Kernel};

fn main() {
	let mut lee_shell = Kernel::new();
	lee_shell.work();
}
