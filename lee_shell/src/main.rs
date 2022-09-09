mod kernel;
mod ui;
mod default_env_vars;

use crate::kernel::{Kernel};

fn main() {
	let mut lee_shell = Kernel::new();
	lee_shell.work();
}
