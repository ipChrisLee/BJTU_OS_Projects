extern crate pest;
#[macro_use]
extern crate pest_derive;

mod command;
mod config;
mod history;
mod kernel;
mod lsh_parser;
mod runner;
mod ui;

use crate::config::LOG4RS_LSH_PATH;
use crate::kernel::Kernel;
use log::info;
use log4rs;

fn main() {
    log4rs::init_file(LOG4RS_LSH_PATH, Default::default()).unwrap();
    let mut lee_shell = Kernel::new();
    lee_shell.work();
}
