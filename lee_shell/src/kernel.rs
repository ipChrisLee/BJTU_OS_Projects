use crate::history::History;
use crate::lsh_parser::{parse_string, Command};
use crate::runner::run;
use crate::ui::UI;
use std::env::{current_dir, set_current_dir, var};
use std::path::PathBuf;
use std::{convert, env};

pub struct Kernel {
    pub ui: UI,
    pub history: History,
}

impl Kernel {
    /// Expand env variable and erase '\n'.
    fn preprocess_script(&self, script: &String) -> String {
        let mut script = script.clone();
        for (name, val) in env::vars() {
            let from = format!("${{{}}}", name);
            script = script.replace(from.as_str(), val.as_str());
        }
        script.trim().to_string()
    }
    pub fn new() -> Kernel {
        Kernel {
            ui: UI::new(),
            history: History::new(),
        }
    }
    pub fn work(&mut self) {
        loop {
            let script = self.ui.get_input();
            let script = self.preprocess_script(&script);
            let cmds = parse_string(script.as_str());
            let mut converted_cmds = Vec::<Command>::new();
            for cmd in cmds {
                dbg!(cmd.clone());
                if let Some(i) = cmd.to_history() {
                    let cmd_s = self.history.get_n_history(i).unwrap();
                    converted_cmds.extend(parse_string(cmd_s.as_str()));
                } else {
                    converted_cmds.push(cmd);
                }
            }
            let mut history_s = String::new();
            converted_cmds
                .iter()
                .for_each(|c| history_s.push_str(c.to_string().as_str()));
            dbg!(history_s.clone());
            self.history.add_history(history_s);
            run(self, converted_cmds);
        }
    }
}
