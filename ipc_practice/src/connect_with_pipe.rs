use std::cell::RefCell;
use std::char::ParseCharError;
use std::io::{stdin, stdout};
use std::ops::Add;
use std::rc::Rc;

use log::info;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::fstat;
use nix::sys::stat::Mode;
use nix::sys::wait::waitpid;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{close, dup2, fork, pipe, read, sleep, write};
use std::process::exit;

pub fn pipe_navie(src_file_path: &str, dst_file_path: &str, target_str: &str) {
    let pipe_from_parent_to_child = pipe().unwrap();
    let pipe_from_child_to_parent = pipe().unwrap();
    let pid = unsafe { fork() }.unwrap();
    match pid {
        Parent { child } => {
            close(pipe_from_parent_to_child.0).unwrap();
            close(pipe_from_child_to_parent.1).unwrap();
            //  Redirect stdin to src file.
            let fin = open(src_file_path, OFlag::O_RDONLY, Mode::S_IRUSR).unwrap();
            dup2(fin, 0).unwrap();
            close(fin).unwrap();
            //  Open target file.
            let fout = open(
                dst_file_path,
                OFlag::O_RDWR | OFlag::O_CREAT | OFlag::O_TRUNC,
                Mode::S_IWUSR | Mode::S_IRUSR,
            )
            .unwrap();
            //  Read from stdin, transfer to upper pipe.
            let mut buf = String::new();
            while stdin().read_line(&mut buf).unwrap() > 0 {
                write(pipe_from_parent_to_child.1, buf.as_bytes()).unwrap();
                buf.clear();
            }
            buf.clear();
            close(pipe_from_parent_to_child.1).unwrap();
            //  Redirect stdin to lower pipe to get result from child.
            dup2(pipe_from_child_to_parent.0, 0).unwrap();
            close(pipe_from_child_to_parent.0).unwrap();
            //  Read from lower pipe and transfer to stdout.
            info!("Parent will read from child.");
            while stdin().read_line(&mut buf).unwrap() > 0 {
                write(fout, buf.as_bytes()).unwrap();
                // dbg!(buf.clone());
                buf.clear();
            }
            info!("Parent finished reading from child.");
            //  Wait child to finish.
            waitpid(child, None).unwrap();
        }
        Child => {
            close(pipe_from_parent_to_child.1).unwrap();
            close(pipe_from_child_to_parent.0).unwrap();
            //  Redirect stdin to upper pipe.
            dup2(pipe_from_parent_to_child.0, 0).unwrap();
            close(pipe_from_parent_to_child.0).unwrap();
            //  Read from parent, and filter, and write result to lower pipe.
            info!("Child will begin processing.");
            let mut buf = String::new();
            while stdin().read_line(&mut buf).unwrap() > 0 {
                if buf.split_whitespace().any(|word| word.eq(target_str)) {
                    write(pipe_from_child_to_parent.1, buf.as_bytes()).unwrap();
                }
                // dbg!(buf.clone());
                buf.clear();
            }
            close(pipe_from_child_to_parent.1).unwrap();
            info!("Child finished processing.");
            //  Finish.
            exit(0);
        }
    }
}

pub fn pipe_with_big_buffer(src_file_path: &str, dst_file_path: &str, target_str: &str) {
    let pipe_from_parent_to_child = pipe().unwrap();
    let pipe_from_child_to_parent = pipe().unwrap();
    let pid = unsafe { fork() }.unwrap();
    match pid {
        Parent { child } => {
            close(pipe_from_parent_to_child.0).unwrap();
            close(pipe_from_child_to_parent.1).unwrap();
            //  Redirect stdin to src file.
            let fin = open(src_file_path, OFlag::O_RDONLY, Mode::S_IRUSR).unwrap();
            dup2(fin, 0).unwrap();
            close(fin).unwrap();
            //  Open target file.
            let fout = open(
                dst_file_path,
                OFlag::O_RDWR | OFlag::O_CREAT | OFlag::O_TRUNC,
                Mode::S_IWUSR | Mode::S_IRUSR,
            )
            .unwrap();
            //  Read from stdin, transfer to upper pipe.
            let mut buf = String::new();
            while stdin().read_line(&mut buf).unwrap() > 0 {
                write(pipe_from_parent_to_child.1, buf.as_bytes()).unwrap();
                buf.clear();
            }
            buf.clear();
            close(pipe_from_parent_to_child.1).unwrap();
            //  Redirect stdin to lower pipe to get result from child.
            dup2(pipe_from_child_to_parent.0, 0).unwrap();
            close(pipe_from_child_to_parent.0).unwrap();
            //  Read from lower pipe and transfer to stdout.
            info!("Parent will read from child.");
            while stdin().read_line(&mut buf).unwrap() > 0 {
                write(fout, buf.as_bytes()).unwrap();
                // dbg!(buf.clone());
                buf.clear();
            }
            info!("Parent finished reading from child.");
            //  Wait child to finish.
            waitpid(child, None).unwrap();
        }
        Child => {
            close(pipe_from_parent_to_child.1).unwrap();
            close(pipe_from_child_to_parent.0).unwrap();
            //  Redirect stdin to upper pipe.
            dup2(pipe_from_parent_to_child.0, 0).unwrap();
            close(pipe_from_parent_to_child.0).unwrap();
            //  Read from parent, but not filter.
            info!("Child will begin to read from parent and filter and transfer.");
            let mut buf = String::new();
            let mut res = String::new();
            while stdin().read_line(&mut buf).unwrap() > 0 {
                if buf.split_whitespace().any(|word| word.eq(target_str)){
                    res.push_str(buf.as_str());
                }
                buf.clear();
            }
            write(pipe_from_child_to_parent.1, res.as_bytes()).unwrap();
            //  Finish.
            exit(0);
        }
    }
}

pub fn pipe_half_duplex(src_file_path: &str, dst_file_path: &str, target_str: &str) {
    let pipe_from_parent_to_child = pipe().unwrap();
    let pipe_from_child_to_parent = pipe().unwrap();
    let pipe_communication = pipe().unwrap();
    let pid = unsafe { fork() }.unwrap();
    match pid {
        Parent { child } => {
            close(pipe_from_parent_to_child.0).unwrap();
            close(pipe_from_child_to_parent.1).unwrap();
            close(pipe_communication.1).unwrap();
            //  Redirect stdin to src file.
            let fin = open(src_file_path, OFlag::O_RDONLY, Mode::S_IRUSR).unwrap();
            dup2(fin, 0).unwrap();
            close(fin).unwrap();
            //  Open target file.
            let fout = open(
                dst_file_path,
                OFlag::O_RDWR | OFlag::O_CREAT | OFlag::O_TRUNC,
                Mode::S_IWUSR | Mode::S_IRUSR,
            )
            .unwrap();
            //  Main loop.
            let mut line = 0;
            loop {
                //  Read one line from stdin, transfer to upper pipe.
                let mut buf = String::new();
                if stdin().read_line(&mut buf).unwrap() > 0 {
                    if !buf.ends_with('\n') {
                        buf.push('\n');
                    }
                    write(pipe_from_parent_to_child.1, buf.as_bytes()).unwrap();
                    line = line + 1;
                    // info!("P : buf.len()={}", buf.len());
                } else {
                    break;
                }
                // info!("P : line={}", line);
                let mut sig = [0 as u8];
                //  Wait for child finishing processing.
                if read(pipe_communication.0, &mut sig).unwrap() > 0 {
                    // info!("P : sig={}", sig[0]);
                    if sig[0] != 0 {
                        //  Something need to be printed.
                        let mut buf = String::new();
                        let mut pack = [0 as u8; 4096];
                        while let Ok(len) = read(pipe_from_child_to_parent.0, &mut pack) {
                            buf.push_str(std::str::from_utf8(&pack[0..len]).unwrap());
                            if pack[len - 1] == '\n' as u8 {
                                break;
                            }
                        }
                        write(fout, buf.as_bytes()).unwrap();
                    }
                } else {
                    panic!("Fail to get info from communication pipe.");
                }
            }
            close(pipe_from_parent_to_child.1).unwrap();
            //  Wait child to finish.
            waitpid(child, None).unwrap();
        }
        Child => {
            close(pipe_from_parent_to_child.1).unwrap();
            close(pipe_from_child_to_parent.0).unwrap();
            close(pipe_communication.0).unwrap();
            //  Redirect stdin to upper pipe.
            dup2(pipe_from_parent_to_child.0, 0).unwrap();
            close(pipe_from_parent_to_child.0).unwrap();
            //  Predefine of some useful things.
            let sig_found = [1 as u8];
            let sig_not_found = [0 as u8];
            //  Main loop.
            loop {
                let mut buf = String::new();
                if stdin().read_line(&mut buf).unwrap() > 0 {
                    // info!("C : buf.len()={}", buf.len());
                    if buf.split_whitespace().any(|s| s.eq(target_str)) {
                        // info!("C : sig=sig_found");
                        write(pipe_communication.1, &sig_found).unwrap();
                        write(pipe_from_child_to_parent.1, buf.as_bytes()).unwrap();
                    } else {
                        // info!("C : sig=sig_not_found");
                        write(pipe_communication.1, &sig_not_found).unwrap();
                    }
                    buf.clear();
                } else {
                    break;
                }
            }
            close(pipe_from_child_to_parent.1).unwrap();
            close(pipe_communication.1).unwrap();
            //  Finish.
            exit(0);
        }
    }
}
