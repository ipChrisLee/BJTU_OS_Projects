use crate::lsh_parser::{CmdType, Command};
use nix::errno::Errno;
use nix::fcntl::{open, OFlag};
use nix::sys::signal::kill;
use nix::sys::signal::Signal::SIGKILL;
use nix::sys::stat::Mode;
use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{chdir, close, dup, dup2, execv, execvp, fork};
use nix::unistd::{pipe, ForkResult,Pid};
use std::env::{current_dir, set_current_dir, var};
use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::vec::Vec;

const SHOULD_RUN_ON_THIS: &[&'static str] = &["cd", "exit"];

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
fn run_builtin_command(cmd_name: String, args: Vec<String>) {
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

fn run_outer_command(cmd_name: String, args: Vec<String>) {
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

fn run_spec_exe_command(cmd_name: String, args: Vec<String>) {
    let p_exe = PathBuf::from(cmd_name);
    let p_exe = if p_exe.is_relative() {
        let mut p = PathBuf::from(current_dir().unwrap());
        p.join(p_exe);
        p
    } else {
        p_exe
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

//  Run command without context switch.
fn run_command(cmd: Command) {
    if let Some(redirect_in) = cmd.redirect_in {
        let fd = open(
            redirect_in.as_os_str(),
            OFlag::O_RDONLY,
            Mode::S_IRUSR | Mode::S_IWUSR,
        )
        .unwrap();
        dup2(fd, 0).unwrap();
        close(fd).unwrap();
    }
    if let Some(redirect_out) = cmd.redirect_out {
        let fd = open(
            redirect_out.as_os_str(),
            OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC,
            Mode::S_IRUSR | Mode::S_IWUSR,
        )
        .unwrap();
        dup2(fd, 1).unwrap();
        close(fd).unwrap();
    }
    match cmd.cmd_type {
        CmdType::Builtin(cmd_name) => run_builtin_command(cmd_name, cmd.args),
        CmdType::Outer(cmd_name) => run_outer_command(cmd_name, cmd.args),
        CmdType::SpecExe(cmd_name) => run_spec_exe_command(cmd_name, cmd.args),
    }
}

fn run_commands(commands: Vec<Command>) {
    //  backup stdio
    let fd_stdin = dup(0).unwrap();
    let fd_stdout = dup(1).unwrap();
    let fd_stderr = dup(2).unwrap();
    let mut iter = commands.into_iter();
    let cmd = iter.next().unwrap();
    let followed_cmds: Vec<_> = iter.collect();
    if !followed_cmds.is_empty() {
        let mut pid = Result::<ForkResult, Errno>::Err(Errno::from_i32(-1));
        unsafe {
            pid = fork();
        }
        let pid = pid.expect("Fork Failed: Unable to create child process!");
        let p = pipe().expect("Fail to create pipe.");
        match pid {
            Child => {
                close(p.1).unwrap();
                dup2(p.0, 0).unwrap();
                close(p.0).unwrap();
                run_commands(followed_cmds);
            }
            Parent { child: _ } => {
                close(p.0).unwrap();
                dup2(p.1, 1).unwrap();
                close(p.1).unwrap();
                run_command(cmd);
            }
        }
    } else {
        //  For some commands, it should not run as process.
        let mut have_run = false;
        if let CmdType::Builtin(cmd_name) = cmd.cmd_type.clone() {
            if SHOULD_RUN_ON_THIS.contains(&(cmd_name.as_str())) {
                run_command(cmd.clone());
                have_run = true;
            }
        }
        if !have_run {
            //  For other commands, just run on sub process.
            let mut pid = Result::<ForkResult, Errno>::Err(Errno::from_i32(-1));
            unsafe {
                pid = fork();
            }
            let pid = pid.expect("Fork Failed: Unable to create child process!");
            match pid {
                Parent { child: _ } => {
                    wait().unwrap();
                }
                Child => {
                    run_command(cmd);
                }
            };
        }
    }
    dup2(fd_stdin, 0).unwrap();
    close(fd_stdin).unwrap();
    dup2(fd_stdout, 1).unwrap();
    close(fd_stdout).unwrap();
    dup2(fd_stderr, 2).unwrap();
    close(fd_stderr).unwrap();
}

pub fn run(commands: Vec<Command>) {
    dbg!("Running ", commands.clone());
    if commands.is_empty() {
    } else {
        run_commands(commands);
    }
}
