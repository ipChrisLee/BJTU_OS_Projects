use crate::command::{self, Command};
use crate::history::History;
use crate::lsh_parser::parse_string;
use crate::runner::run;
use crate::ui::UI;
use log::info;
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
            let mut command_group = parse_string(script.as_str());
            if let Some(res) = command_group.to_history_number() {
                let history_command_group_id = res.unwrap();
                info!("using history {:?}",history_command_group_id);
                command_group = self
                    .history
                    .get_n_history(history_command_group_id)
                    .unwrap()
                    .clone();
            }
            info!("{:?}", command_group.clone());
            self.history.add_history(command_group.clone());
            run(self, command_group);
        }
    }
}
