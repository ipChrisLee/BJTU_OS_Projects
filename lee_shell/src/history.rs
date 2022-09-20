use crate::command::{self, CmdType, Command, CommandGroup};
use crate::config::HISTORY_NUMBER;
use crate::kernel::Kernel;
use std::collections::vec_deque::Iter;
use std::iter::Rev;
use std::{collections::VecDeque, iter::Enumerate};

pub struct History {
    history: VecDeque<CommandGroup>,
}

impl History {
    pub fn new() -> History {
        assert!(HISTORY_NUMBER > 2);
        History {
            history: VecDeque::new(),
        }
    }
    pub fn add_history(&mut self, command_group: CommandGroup) {
        self.history.push_front(command_group);
        while self.history.len() > HISTORY_NUMBER + 1 {
            self.history.pop_back();
        }
    }
    pub fn all_history(&self) -> Rev<Enumerate<Iter<CommandGroup>>> {
        self.history.iter().enumerate().rev()
    }
    pub fn get_n_history(&self, n: usize) -> Option<&CommandGroup> {
        self.history.get(n)
    }
    // pub fn convert_history_command(
    //     kernel: Kernel,
    //     previous_commands: CommandGroup,
    // ) -> CommandGroup {
    //     let mut iter = previous_commands.commands.iter();
    //     if let Some(first_cmd) = iter.next() {
    //         if let CmdType::Builtin(ref builtin_cmd_name) = first_cmd.cmd_type {
    //             if let Some(idx) = first_cmd.to_history_number() {
                    
    //             } else {
    //             }
    //         }
    //     }
    //     previous_commands
    // }
}
