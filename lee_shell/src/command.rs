use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, dup2};
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum CmdType {
    Builtin(String),
    Outer(String),
    SpecExe(String),
}

#[derive(Debug, Clone)]
pub struct Command {
    pub cmd_type: CmdType,
    pub args: Vec<String>,
    pub redirect_in: Option<PathBuf>,
    pub redirect_out: Option<PathBuf>,
}

impl Command {
    pub fn redirect(&self) {
        if let Some(ref redirect_in) = self.redirect_in {
            let fd = open(
                redirect_in.as_os_str(),
                OFlag::O_RDONLY,
                Mode::S_IRUSR | Mode::S_IWUSR,
            )
            .unwrap();
            dup2(fd, 0).unwrap();
            close(fd).unwrap();
        }
        if let Some(ref redirect_out) = self.redirect_out {
            let fd = open(
                redirect_out.as_os_str(),
                OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC,
                Mode::S_IRUSR | Mode::S_IWUSR,
            )
            .unwrap();
            dup2(fd, 1).unwrap();
            close(fd).unwrap();
        }
    }
    pub fn to_history_number(&self) -> Option<usize> {
        if let CmdType::Builtin(ref cmd) = self.cmd_type {
            if cmd.eq("!!") {
                Some(0)
            } else if cmd.eq("!") {
                let s = self.args.get(0).unwrap().parse::<usize>().unwrap();
                Some(s)
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        match self.cmd_type {
            CmdType::Builtin(ref c) => s.push_str(c.as_str()),
            CmdType::Outer(ref c) => s.push_str(c.as_str()),
            CmdType::SpecExe(ref c) => s.push_str(c.as_str()),
        }
        for arg in &self.args {
            s.push_str(" \"");
            s.push_str(arg.as_str());
            s.push_str("\"");
        }
        if let Some(ref file_path) = self.redirect_in {
            s.push_str(" < ");
            s.push_str(file_path.to_str().unwrap());
        }
        if let Some(ref file_path) = self.redirect_out {
            s.push_str(" > ");
            s.push_str(file_path.to_str().unwrap());
        }
        s
    }
}

#[derive(Debug, Clone)]
pub struct CommandGroup {
    pub commands: Vec<Command>,
    pub on_background: bool,
}
impl CommandGroup {
    pub fn new() -> CommandGroup {
        CommandGroup {
            commands: Vec::new(),
            on_background: false,
        }
    }
    pub fn to_string(&self) -> String {
        let mut res = if self.commands.is_empty() {
            String::new()
        } else {
            let mut iter = self.commands.iter();
            let mut s = String::new();
            s = iter.next().unwrap().to_string();
            while let Some(cmd) = iter.next() {
                s.push_str(" | ");
                s.push_str(cmd.to_string().as_str());
            }
            s
        };
        if self.on_background {
            res.push_str(" &");
        }
        res
    }
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
    pub fn to_history_number(&self) -> Option<Result<usize, &'static str>> {
        let mut lst_idx = -1;
        let mut lst_history_idx = -1;
        for (idx, cmd) in self.commands.iter().enumerate() {
            if let Some(history_idx) = cmd.to_history_number() {
                if cmd.redirect_in.is_some() || cmd.redirect_out.is_some() {
                    return Some(Err(""));
                }
                lst_idx = idx as i32;
                lst_history_idx = history_idx as i32;
            }
        }
        if lst_idx == -1 {
            None
        } else if lst_idx == 0 && self.commands.len() == 1 && !self.on_background {
            Some(Ok(lst_history_idx as usize))
        } else {
            Some(Err(""))
        }
    }
}
