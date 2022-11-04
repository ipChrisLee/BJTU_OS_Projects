use std::process::exit;

use anyhow::Context;
use interprocess::local_socket;
use interprocess::local_socket::{LocalSocketListener, LocalSocketStream, NameTypeSupport};
use nix::sys::wait::waitpid;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{close, dup2, fork, pipe, read, write};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{
    io::{self, BufReader},
    sync::mpsc::Sender,
};
fn server(src_file_path: &str, dst_file_path: &str) {
    fn handle_error(conn: io::Result<LocalSocketStream>) -> Option<LocalSocketStream> {
        match conn {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("Incoming connection failed: {}", e);
                None
            }
        }
    }

    let name = {
        use NameTypeSupport::*;
        match NameTypeSupport::query() {
            OnlyPaths => "demo/tmp.sock",
            OnlyNamespaced | Both => "@tmp.sock",
        }
    };

    let listener = LocalSocketListener::bind(name).unwrap();

    for conn in listener.incoming().filter_map(handle_error) {
        let mut conn = BufReader::new(conn);

        let path = Path::new(src_file_path);
        let mut fin = File::open(&path).unwrap();
        let mut buf = String::new();
        fin.read_to_string(&mut buf).unwrap();
        conn.get_mut().write_all(&buf.len().to_be_bytes()).unwrap();
        conn.get_mut().write_all(buf.as_bytes()).unwrap();

        buf.clear();
        conn.read_to_string(&mut buf).unwrap();
        // dbg!(buf.clone());
        let path = Path::new(dst_file_path);
        let mut fout = File::create(&path).unwrap();
        fout.write_all(buf.as_bytes()).unwrap();
        break;
    }
}
fn client(target_str: &str) {
    let name = {
        use NameTypeSupport::*;
        match NameTypeSupport::query() {
            OnlyPaths => "demo/tmp.sock",
            OnlyNamespaced | Both => "@tmp.sock",
        }
    };

    let conn = LocalSocketStream::connect(name).unwrap();
    let mut conn = BufReader::new(conn);

    let mut len = {
        let mut len = [0 as u8; 8];
        conn.read_exact(&mut len).unwrap();
        usize::from_be_bytes(len)
    };
    // dbg!(len);
    let mut len_now = 0 as usize;

    let mut buf = String::new();
    let mut ch_buf = [0 as u8];
    while conn.read(&mut ch_buf).unwrap() > 0 {
        buf.push(ch_buf[0] as char);
        len_now = len_now + 1;
        if len == len_now {
            break;
        }
    }

    // dbg!(buf.clone());
    let mut res = String::new();
    let res = &mut res;
    buf.split("\n")
        .filter(|line| (*line).split_whitespace().any(|word| word.eq(target_str)))
        .for_each(|line| {
            res.push_str(line);
            res.push('\n');
        });
    // dbg!(res.clone());
    conn.get_mut().write_all(res.as_bytes()).unwrap();
}
pub fn socket_method(src_file_path: &str, dst_file_path: &str, target_str: &str) {
    let pid = unsafe { fork() }.unwrap();
    match pid {
        Parent { child } => {
            server(src_file_path, dst_file_path);
            waitpid(child, None).unwrap();
            std::fs::remove_file("demo/tmp.sock").unwrap();
        }
        Child => {
            client(target_str);
            exit(0);
        }
    }
}
