use std::env;
use pest::Parser;
use pest::iterators;
use crate::common::print_type_name;

#[derive(Parser)]
#[grammar = "lsh.pest"]
pub struct LSHParser;

#[derive(Debug)]
enum CmdType {
	Builtin(String),
	Outer(String),
	SpecExe(String),
}

#[derive(Debug)]
pub struct Command {
	cmd_type: CmdType,
	args: Vec<String>,
	redirect_in: Option<String>,
	redirect_out: Option<String>,
}

impl Command {
	fn new(cmd_type: CmdType) -> Command {
		Command {
			cmd_type,
			args: Vec::new(),
			redirect_in: Option::None,
			redirect_out: Option::None,
		}
	}
}

fn parse_pipe_cmd() -> Vec<Command> {
	Vec::new()
}

pub fn parse_string(input: &str) -> Vec<Command> {
	let parse_root = LSHParser::parse(Rule::SCRIPT, input).expect("Illegal command!");
	dbg!(parse_root.clone());
	for pair in parse_root {
		match pair.as_rule() {
			Rule::PIPE_CMD => {
				let a=pair.into_inner();
				print_type_name(a);
				println!("{}",pair.into_inner());
				// parse_pipe_cmd(pair);
			}
			_ => {
				println!("??");
			}
		}
	}
	Vec::new()
}
