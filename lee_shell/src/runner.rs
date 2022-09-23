use crate::command::{self, CmdType, Command, CommandGroup};
use crate::kernel::{self, Kernel};
use core::panic;
use log::info;
use nix::errno::Errno;
use nix::fcntl::{open, OFlag};
use nix::sys::signal::kill;
use nix::sys::signal::Signal::SIGKILL;
use nix::sys::stat::Mode;
use nix::sys::wait::{wait, waitpid};
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{chdir, close, dup, dup2, execv, execvp, fork};
use nix::unistd::{pipe, ForkResult, Pid};
use std::env::{current_dir, set_current_dir, var};
use std::ffi::{CStr, CString};
use std::io::stdin;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::vec::Vec;

fn find_command(cmd_name: String) -> Option<PathBuf> {
    let path_var = var("PATH").unwrap();
    let paths = path_var.split(":").collect::<Vec<_>>();
    // dbg!(paths.clone());
    for path_base in paths {
        let mut full_path = String::from(path_base);
        full_path.push_str("/");
        full_path.push_str(cmd_name.as_str());
        let p = PathBuf::from(full_path);
        if p.exists() {
            return Some(p);
        }
    }
    None
}
fn run_builtin_command_in_place(kernel: &Kernel, cmd_name: String, args: Vec<String>) {
    match cmd_name.as_str() {
        "cd" => {
            let dir = if let Some(cd_dir) = args.get(0) {
                cd_dir.clone()
            } else {
                String::from(var("HOME").expect("Missing $HOME env var."))
            };
            let cd_path = PathBuf::from(dir);
            set_current_dir(cd_path).unwrap();
        }
        "pwd" => {
            let p = current_dir().unwrap();
            println!("{}", p.as_path().to_str().unwrap());
        }
        "exit" => {
            let exit_code = args.get(0);
            let exit_code = if let Some(s) = exit_code {
                s.parse::<i32>().unwrap()
            } else {
                0
            };
            exit(exit_code);
        }
        "echo" => {
            let mut buf = String::new();
            for arg in args {
                buf.push_str(arg.as_str());
                buf.push_str(" ")
            }
            println!("{}", buf);
        }
        "kill" => {
            let pid = args.get(0);
            let pid = if let Some(s) = pid {
                s.parse::<i32>().unwrap()
            } else {
                0
            };
            kill(Pid::from_raw(pid), SIGKILL).unwrap();
        }
        "history" => {
            kernel
                .history
                .all_history()
                .for_each(|(i, c)| println!("{} : {}", i, c.to_string()));
        }
        s => todo!("todo builtin {}", s),
    }
}

fn run_outer_command_in_place(kernel: &Kernel, cmd_name: String, args: Vec<String>) {
    // dbg!(cmd_name.clone());
    let p_exe = find_command(cmd_name).unwrap();
    let p_exe = p_exe.as_path().to_str().unwrap();
    let mut argv = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    argv.insert(0, p_exe);
    let argv = argv
        .into_iter()
        .map(|s| CString::new(s).unwrap())
        .collect::<Vec<_>>();
    let argv = argv.iter().map(|s| s.as_c_str()).collect::<Vec<_>>();
    // dbg!(argv.clone());
    execv(argv[0], argv.as_slice()).unwrap();
}

fn run_spec_exe_command_in_place(kernel: &Kernel, cmd_name: String, args: Vec<String>) {
    let p_exe = PathBuf::from(cmd_name);
    let p_exe = if p_exe.is_relative() {
        let mut p = PathBuf::from(current_dir().unwrap());
        p = p.join(p_exe);
        p
    } else if p_exe.is_absolute() {
        p_exe
    } else {
        panic!();
    };
    let p_exe = p_exe.as_path().to_str().unwrap();
    let mut argv = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    argv.insert(0, p_exe);
    let argv = argv
        .into_iter()
        .map(|s| CString::new(s).unwrap())
        .collect::<Vec<_>>();
    let argv = argv.iter().map(|s| s.as_c_str()).collect::<Vec<_>>();
    execv(argv[0], argv.as_slice()).unwrap();
}

//  For builtin: This will return to parent process.
//  Otherwise  : This will not return to parent process.
//  This will redirect io.
fn run_command_in_place(kernel: &Kernel, cmd: Command) {
    cmd.redirect();
    match cmd.cmd_type {
        CmdType::Builtin(cmd_name) => run_builtin_command_in_place(kernel, cmd_name, cmd.args),
        CmdType::Outer(cmd_name) => run_outer_command_in_place(kernel, cmd_name, cmd.args),
        CmdType::SpecExe(cmd_name) => run_spec_exe_command_in_place(kernel, cmd_name, cmd.args),
    }
}

//  For all command: This will return to parent process. But builtin will not work.
//  This will redirect io.
//  You can choose wait or not wait for subprocess end.
fn run_command_out_of_place(kernel: &Kernel, cmd: Command, wait_for_end: bool) {
    let pid = unsafe { fork() }.unwrap();
    match pid {
        Parent { child } => {
            if wait_for_end {
                waitpid(child, None).unwrap();
            }
        }
        Child => {
            run_command_in_place(kernel, cmd);
            exit(0);
        }
    }
}

//  For all command: This will return to parent process. And builtin will work.
//  This will redirect io. So you should notice io for builtin(Since this will change the performance of shell).
//  This will return until subprocess end.
fn run_command_combined(kernel: &Kernel, command: Command) {
    command.redirect();
    if let CmdType::Builtin(ref cmd_name) = command.cmd_type {
        run_builtin_command_in_place(kernel, cmd_name.clone(), command.args);
        return;
    }
    let pid = unsafe { fork() }.unwrap();
    match pid {
        Parent { child } => {
            waitpid(child, None).unwrap();
        }
        Child => {
            match command.cmd_type {
                CmdType::Outer(cmd_name) => {
                    run_outer_command_in_place(kernel, cmd_name, command.args)
                }
                CmdType::SpecExe(cmd_name) => {
                    run_spec_exe_command_in_place(kernel, cmd_name, command.args)
                }
                _ => panic!(),
            };
            exit(0);
        }
    }
}

fn run_single_command_on_frontend(kernel: &Kernel, command: Command) {
    let fd_stdin = dup(0).unwrap();
    let fd_stdout = dup(1).unwrap();
    let fd_stderr = dup(2).unwrap();
    run_command_combined(kernel, command);
    dup2(fd_stdin, 0).unwrap();
    close(fd_stdin).unwrap();
    dup2(fd_stdout, 1).unwrap();
    close(fd_stdout).unwrap();
    dup2(fd_stderr, 2).unwrap();
    close(fd_stderr).unwrap();
}
fn run_single_command_on_background(kernel: &Kernel, command: Command) {
    run_command_out_of_place(kernel, command, false);
}

fn run_multi_commands(kernel: &Kernel, commands: Vec<Command>, wait_for_end: bool) {
    // info!("Running commands. wait_for_end = {}",wait_for_end);
    let pid = unsafe { fork() }.unwrap();
    match pid {
        Parent { child } => {
            if wait_for_end {
                waitpid(child, None).unwrap();
            }
        }
        Child => {
            let mut childs_pid = Vec::new();
            let mut iter = commands.into_iter().peekable();
            let mut pre_out = 0;
            while let Some(command) = iter.next() {
                if iter.peek().is_none() {
                    //  the last
                    let pid = unsafe { fork() }.unwrap();
                    match pid {
                        Parent { child } => {
                            childs_pid.push(child);
                            close(pre_out).unwrap();
                        }
                        Child => {
                            dup2(pre_out, 0).unwrap();
                            run_command_in_place(kernel, command);
                            exit(0);
                        }
                    }
                } else {
                    let p = pipe().unwrap();
                    let pid = unsafe { fork() }.unwrap();
                    match pid {
                        Parent { child } => {
                            childs_pid.push(child);
                            close(p.1).unwrap();
                            if pre_out != 0 {
                                close(pre_out).unwrap();
                            }
                            pre_out = p.0;
                        }
                        Child => {
                            close(p.0).unwrap();
                            dup2(pre_out, 0).unwrap();
                            dup2(p.1, 1).unwrap();
                            run_command_in_place(kernel, command);
                            exit(0);
                        }
                    }
                }
            }
            if wait_for_end {
                for pid in childs_pid {
                    waitpid(pid, None).unwrap();
                }
            }
            exit(0);
        }
    }
}

pub fn run(kernel: &Kernel, commands: CommandGroup) {
    let on_background = commands.on_background;
    let commands = commands.commands;
    info!("{:?}", commands.clone());
    if commands.is_empty() {
        // info!("Empty!");
    } else if commands.len() == 1 {
        // info!("Single!");
        let command = commands.into_iter().next().unwrap();
        if on_background {
            run_single_command_on_background(kernel, command);
        } else {
            run_single_command_on_frontend(kernel, command);
        }
    } else {
        // info!("Multi!");
        run_multi_commands(kernel, commands, !on_background);
    }
}
