use core::panic;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::path::PathBuf;

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
    redirect_in: Option<PathBuf>,
    redirect_out: Option<PathBuf>,
}

fn parse_cmd_name(cmd_name_pair: Pair<Rule>) -> CmdType {
    let mut pairs=cmd_name_pair.into_inner();
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::BUILTIN_CMD => CmdType::Builtin(pair.as_str().to_string()),
        Rule::OUTER_CMD => CmdType::Outer(pair.as_str().to_string()),
        Rule::SPEC_EXE => CmdType::SpecExe(pair.as_str().to_string()),
        a => panic!("{:?}", a),
    }
}

fn parse_arg(arg_str_pair: Pair<Rule>) -> String {
    arg_str_pair.as_str().to_string()
}

fn parse_redirect(redirect_info_pair: Pair<Rule>) -> (Option<PathBuf>, Option<PathBuf>) {
    let mut redirect_in = Option::<PathBuf>::None;
    let mut redirect_out = Option::<PathBuf>::None;
    for pair in redirect_info_pair.into_inner() {
        match pair.as_rule() {
            Rule::CHANGE_INPUT => {
                let pair=pair.into_inner().next().unwrap();
                redirect_in = Some(PathBuf::from(pair.as_str()))
            }
            Rule::CHANGE_OUTPUT => {
                let pair=pair.into_inner().next().unwrap();
                redirect_out = Some(PathBuf::from(pair.as_str()))
            }
            a => panic!("{:?}", a),
        }
    }
    (redirect_in, redirect_out)
}

fn parse_cmd(cmd_pair: Pair<Rule>) -> Command {
    let mut cmd_name = Option::<CmdType>::None;
    let mut args = Vec::<String>::new();
    let mut redirect_in = Option::<PathBuf>::None;
    let mut redirect_out = Option::<PathBuf>::None;
    for pair in cmd_pair.into_inner() {
        match pair.as_rule() {
            Rule::CMD_NAME => cmd_name = Some(parse_cmd_name(pair)),
            Rule::ARG => args.push(parse_arg(pair)),
            Rule::REDIRECT_IO => (redirect_in, redirect_out) = parse_redirect(pair),
            a=> panic!("{:?}", a),
        }
    }
    Command {
        cmd_type: cmd_name.unwrap(),
        args: args,
        redirect_in: redirect_in,
        redirect_out: redirect_out,
    }
}

fn parse_pipe_cmd(pairs: Pairs<Rule>) -> Vec<Command> {
    pairs.map(|pair| parse_cmd(pair)).collect()
}

pub fn parse_string(input: &str) -> Vec<Command> {
    let parse_root = LSHParser::parse(Rule::SCRIPT, input).expect("Illegal command!");
    for pair in parse_root {
        match pair.as_rule() {
            Rule::PIPE_CMD => {
                return parse_pipe_cmd(pair.into_inner());
            }
            a => panic!("{:?}", a),
        }
    }
    Vec::new()
}
