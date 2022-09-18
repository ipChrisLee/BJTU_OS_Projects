extern crate pest;
#[macro_use]
extern crate pest_derive;

mod config;
mod history;
mod kernel;
mod lsh_parser;
mod runner;
mod ui;

use crate::kernel::Kernel;

fn main() {
    let mut lee_shell = Kernel::new();
    lee_shell.work();
}
