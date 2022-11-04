use log::info;
use nix::errno::Errno;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::sys::wait::waitpid;
use nix::unistd::{
    close, dup2, fork, mkfifo, pipe, read, sleep, write,
    ForkResult::{Child, Parent},
};
use std::ffi::CString;
use std::fs;
use std::io::{stdin, stdout};
use std::process::exit;

static FIFO_P2C_PATH: &'static str = "demo/fifo_p2c";
static FIFO_C2P_PATH: &'static str = "demo/fifo_c2p";
const IN_BUF_SIZE: usize = 1024;

fn parent_part(src_file_path: &str, dst_file_path: &str, fd_p2c_write: i32, fd_c2p_read: i32) {
    let fin = open(src_file_path, OFlag::O_RDONLY, Mode::S_IRUSR).unwrap();
    let fout = open(
        dst_file_path,
        OFlag::O_CREAT | OFlag::O_WRONLY,
        Mode::S_IRUSR | Mode::S_IWUSR,
    )
    .unwrap();
    let mut in_buf = [0 as u8; IN_BUF_SIZE];
    let mut lower_pipe_buf = [0 as u8; IN_BUF_SIZE];
    let mut buffered_len = 0;
    let mut end_of_src = false;
    let mut finished = false;
    while !finished {
        // info!("P : Loop");
        //  Try to transfer data from src to child.
        if buffered_len > 0 {
            if let Ok(_) = write(fd_p2c_write, &in_buf[0..buffered_len]) {
                buffered_len = 0;
            }
        }
        // info!("P : First");
        if buffered_len == 0 && !end_of_src {
            // info!("P : Tranfsering from src to child.");
            //  Nothing is buffered, going transfering.
            loop {
                if let Ok(len) = read(fin, &mut in_buf) {
                    // info!("P : Succeed reading[{}] from src.", len);
                    if len == 0 {
                        end_of_src = true;
                        close(fd_p2c_write).unwrap();
                        break;
                    }
                    buffered_len = len;
                    if let Ok(len) = write(fd_p2c_write, &in_buf[0..buffered_len]) {
                        // info!("P : Succeed writing[{}] to child.", len);
                        buffered_len = 0;
                    } else {
                        break;
                    }
                } else {
                    panic!("P : Read failed.");
                }
            }
        }
        // info!("P : Recieving from child.");
        loop {
            match read(fd_c2p_read, &mut lower_pipe_buf) {
                Ok(len) => {
                    // info!("P : Received[{}]", len);
                    if len == 0 {
                        finished = true;
                        break;
                    }
                    let len = write(fout, &lower_pipe_buf[0..len]).unwrap();
                    // info!("P : Write[{}] to dst.", len);
                }
                Err(err) => {
                    // info!("P : Err[{}] to read from child.", err);
                    // if err == Errno::EAGAIN {
                    //     finished = true;
                    // }
                    break;
                }
            }
        }
    }
}

fn child_part(target_str: &str, fd_p2c_read: i32, fd_c2p_write: i32) {
    //  Redirect stdin to pipe from parent.
    dup2(fd_p2c_read, 0).unwrap();
    close(fd_p2c_read).unwrap();
    //  Redirect stdout to pipe to parent.
    dup2(fd_c2p_write, 1).unwrap();
    close(fd_c2p_write).unwrap();
    //  For every line, filter and transfer.
    let mut buf = String::new();
    while stdin().read_line(&mut buf).unwrap() > 0 {
        // info!("C : loop[{}]", buf.len());
        if buf.split_whitespace().any(|word| word.eq(target_str)) {
            print!("{}", buf);
        }
        buf.clear();
    }
    close(0).unwrap();
}

pub fn pipe_duplex(src_file_path: &str, dst_file_path: &str, target_str: &str) {
    mkfifo(FIFO_P2C_PATH, Mode::S_IRWXU).unwrap();
    mkfifo(FIFO_C2P_PATH, Mode::S_IRWXU).unwrap();
    let p = pipe().unwrap();

    let pid = unsafe { fork() }.unwrap();
    match pid {
        Parent { child } => {
            let fd_c2p_read = open(
                FIFO_C2P_PATH,
                OFlag::O_RDONLY | OFlag::O_NONBLOCK,
                Mode::S_IRUSR,
            )
            .unwrap();
            dbg!("c2p_read");
            // close(p.0).unwrap();
            // write(p.1, &[0 as u8]).unwrap();
            let fd_p2c_write;
            loop {
                let r = open(
                    FIFO_P2C_PATH,
                    OFlag::O_WRONLY | OFlag::O_NONBLOCK,
                    Mode::S_IWUSR,
                );
                if let Ok(i) = r {
                    fd_p2c_write = i;
                    break;
                }
            }
            dbg!("p2c_write");
            parent_part(src_file_path, dst_file_path, fd_p2c_write, fd_c2p_read);
            waitpid(child, None).unwrap();
        }
        Child => {
            let fd_p2c_read = open(FIFO_P2C_PATH, OFlag::O_RDONLY, Mode::S_IRUSR).unwrap();
            dbg!("p2c_read");
            // close(p.1).unwrap();
            // {
            //     let mut b = [0 as u8];
            //     read(p.0, &mut b).unwrap();
            // }
            let fd_c2p_write = open(FIFO_C2P_PATH, OFlag::O_WRONLY, Mode::S_IWUSR).unwrap();
            dbg!("c2p_write");
            child_part(target_str, fd_p2c_read, fd_c2p_write);
            // info!("C : Finished");
            exit(0);
        }
    }

    fs::remove_file(FIFO_P2C_PATH).unwrap();
    fs::remove_file(FIFO_C2P_PATH).unwrap();
}
