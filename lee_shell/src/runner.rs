use crate::kernel::Kernel;
use crate::lsh_parser::{CmdType, Command};
use core::panic;
use nix::errno::Errno;
use nix::fcntl::{open, OFlag};
use nix::sys::signal::kill;
use nix::sys::signal::Signal::SIGKILL;
use nix::sys::stat::Mode;
use nix::sys::wait::wait;
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
fn run_builtin_command(kernel: &Kernel, cmd_name: String, args: Vec<String>) {
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
        s => todo!("todo builtin {}", s),
    }
}

fn run_outer_command(kernel: &Kernel, cmd_name: String, args: Vec<String>) {
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
    dbg!(argv.clone());
    execv(argv[0], argv.as_slice()).unwrap();
}

fn run_spec_exe_command(kernel: &Kernel, cmd_name: String, args: Vec<String>) {
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

//  Run command. This will run on this process(builtin) or replace this process(outer/spec_exe)
fn run_command_from_this(kernel: &Kernel, cmd: Command) {
    match cmd.cmd_type {
        CmdType::Builtin(cmd_name) => run_builtin_command(kernel, cmd_name, cmd.args),
        CmdType::Outer(cmd_name) => run_outer_command(kernel, cmd_name, cmd.args),
        CmdType::SpecExe(cmd_name) => run_spec_exe_command(kernel, cmd_name, cmd.args),
    }
}

fn run_command_without_exiting_this(kernel: &Kernel, cmd: Command) {
    if let CmdType::Builtin(ref _cmd_name) = cmd.cmd_type {
        run_command_from_this(kernel, cmd);
        return;
    }
    //  For other commands, just run on sub process.
    let pid;
    unsafe {
        pid = fork();
    }
    let pid = pid.expect("Fork Failed: Unable to create child process!");
    match pid {
        Parent { child: _ } => {
            wait().unwrap();
        }
        Child => {
            run_command_from_this(kernel, cmd);
            exit(0); //  unreacheable in fact.
        }
    };
}

fn run_multiple_commands(kernel: &Kernel, commands: Vec<Command>) {
    let mut it = commands.into_iter();
    let cmd0 = it.next().unwrap();
    let cmd_rest: Vec<_> = it.collect();
    let p = pipe().unwrap();
    let pid;
    unsafe {
        pid = fork().unwrap();
    }
    match pid {
        Parent { child: _ } => {
            close(p.0).unwrap();
            dup2(p.1, 1).unwrap();
            close(p.1).unwrap();
            run_command_from_this(kernel, cmd0);
            exit(0);
        }
        Child => {
            close(p.1).unwrap();
            dup2(p.0, 0).unwrap();
            close(p.0).unwrap();
            if cmd_rest.len() == 1 {
                println!("??");
                run_command_from_this(kernel, cmd_rest.into_iter().next().unwrap());
                exit(0);
            } else if cmd_rest.len() > 1 {
                run_multiple_commands(kernel, cmd_rest);
            } else {
                panic!("");
            }
        }
    }
}

fn run_single_command(kernel: &Kernel, cmd: Command) {
    cmd.redirect();
    run_command_without_exiting_this(kernel, cmd);
}

pub fn run(kernel: &Kernel, commands: Vec<Command>) {
    dbg!("Running ", commands.clone());
    if commands.is_empty() {
    } else if commands.len() == 1 {
        let fd_stdin = dup(0).unwrap();
        let fd_stdout = dup(1).unwrap();
        let fd_stderr = dup(2).unwrap();

        run_single_command(kernel, commands.into_iter().next().unwrap());

        dup2(fd_stdin, 0).unwrap();
        close(fd_stdin).unwrap();
        dup2(fd_stdout, 1).unwrap();
        close(fd_stdout).unwrap();
        dup2(fd_stderr, 2).unwrap();
        close(fd_stderr).unwrap();
    } else {
        let pid;
        unsafe {
            pid = fork().unwrap();
        }
        match pid {
            Parent { child: _ } => {
                wait().unwrap();
            }
            Child => {
                run_multiple_commands(kernel, commands);
                exit(0);
            }
        }
    }
}
