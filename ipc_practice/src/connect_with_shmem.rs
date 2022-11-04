use crossbeam::scope;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::sys::wait::waitpid;
use nix::unistd::{
    close, fork, pipe, read, write,
    ForkResult::{Child, Parent},
};
use shared_memory::{Shmem, ShmemConf};
use std::fs::File;
use std::io::prelude::*;
use std::mem::{self, size_of};
use std::path::Path;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

static SHMEM_FILE_PATH: &'static str = "demo/shmem";

fn reducer(
    src_file_path: &str,
    dst_file_path: &str,
    control_pipe_parent_to_child_write_end: i32,
    control_pipe_child_to_parent_read_end: i32,
) {
    let path = Path::new(src_file_path);
    let mut fin = File::open(&path).unwrap();
    let mut buf = String::new();
    fin.read_to_string(&mut buf).unwrap();

    let shmem = ShmemConf::new()
        .size(buf.len() + std::mem::size_of::<usize>())
        .flink(SHMEM_FILE_PATH)
        .force_create_flink()
        .create()
        .unwrap();
    let shmem_begin_ptr = shmem.as_ptr();
    let mut raw_ptr = shmem_begin_ptr;
    unsafe {
        let be_u8_arr = buf.len().to_be_bytes();
        for i in 0..mem::size_of::<usize>() {
            std::ptr::write_bytes(raw_ptr, be_u8_arr[i], 1);
            raw_ptr = raw_ptr.add(1);
        }
        for i in 0..buf.len() {
            std::ptr::write_bytes(raw_ptr, (*buf.as_bytes())[i], 1);
            raw_ptr = raw_ptr.add(1);
        }
    }
    write(control_pipe_parent_to_child_write_end, &[0 as u8]).unwrap();
    {
        let mut b = [0 as u8];
        read(control_pipe_child_to_parent_read_end, &mut b).unwrap();
    }

    let shmem = ShmemConf::new()
        .size(std::mem::size_of::<usize>())
        .flink(SHMEM_FILE_PATH)
        .open()
        .unwrap();
    let len = unsafe {
        let mut u8_buf = [0 as u8; std::mem::size_of::<usize>()];
        let mut raw_ptr = shmem.as_ptr();
        for i in 0..std::mem::size_of::<usize>() {
            u8_buf[i] = *raw_ptr;
            raw_ptr = raw_ptr.add(1);
        }
        usize::from_be_bytes(u8_buf)
    };
    let shmem = ShmemConf::new()
        .size(len + std::mem::size_of::<usize>())
        .flink(SHMEM_FILE_PATH)
        .open()
        .unwrap();

    let content = {
        let mut raw_ptr = unsafe { shmem.as_ptr().add(size_of::<usize>()) };
        let mut buf = String::new();
        for _ in 0..len {
            buf.push(unsafe { *raw_ptr as char });
            raw_ptr = unsafe { raw_ptr.add(1) };
        }
        buf
    };
    let path = Path::new(dst_file_path);
    let mut fout = File::create(&path).unwrap();
    fout.write_all(content.as_bytes()).unwrap();
}

fn mapper(
    target_str: &str,
    control_pipe_child_to_parent_write_end: i32,
    control_pipe_parent_to_child_read_end: i32,
) {
    {
        let mut b = [0 as u8];
        read(control_pipe_parent_to_child_read_end, &mut b).unwrap();
    }
    let shmem = ShmemConf::new()
        .size(std::mem::size_of::<usize>())
        .flink(SHMEM_FILE_PATH)
        .open()
        .unwrap();
    let len = unsafe {
        let mut u8_buf = [0 as u8; std::mem::size_of::<usize>()];
        let mut raw_ptr = shmem.as_ptr();
        for i in 0..std::mem::size_of::<usize>() {
            u8_buf[i] = *raw_ptr;
            raw_ptr = raw_ptr.add(1);
        }
        usize::from_be_bytes(u8_buf)
    };
    let shmem = ShmemConf::new()
        .size(len + std::mem::size_of::<usize>())
        .flink(SHMEM_FILE_PATH)
        .open()
        .unwrap();

    let content = {
        let mut raw_ptr = unsafe { shmem.as_ptr().add(size_of::<usize>()) };
        let mut buf = String::new();
        for _ in 0..len {
            buf.push(unsafe { *raw_ptr as char });
            raw_ptr = unsafe { raw_ptr.add(1) };
        }
        buf
    };
    let content = &content;
    let il0 = 0;
    let ir0 = il0 + len / 4;
    let il1 = ir0;
    let ir1 = il1 + len / 4;
    let il2 = ir1;
    let ir2 = il2 + len / 4;
    let il3 = ir2;
    let ir3 = il0 + len;
    let res = Arc::new(Mutex::new(String::new()));
    fn find(idx_begin: usize, idx_br: usize, idx_end: usize, content: &String) -> &str {
        match content[idx_begin..idx_end].find("\n") {
            None => "",
            Some(mut il) => {
                il = idx_begin + il + 1;
                let ir = match content[idx_br..idx_end].find("\n") {
                    None => idx_end,
                    Some(i) => i + idx_br + 1,
                };
                // dbg!((idx_begin, idx_br, idx_end, il, ir));
                &content[il..ir]
            }
        }
    }

    scope(|s| {
        s.spawn(|_| {
            find(il0, ir0, ir3, content)
                .split("\n")
                .filter(|line| line.split_whitespace().any(|word| word.eq(target_str)))
                .for_each(|line| {
                    res.lock().unwrap().push_str(line);
                    res.lock().unwrap().push('\n');
                });
        });
        s.spawn(|_| {
            find(il1, ir1, ir3, content)
                .split("\n")
                .filter(|line| line.split_whitespace().any(|word| word.eq(target_str)))
                .for_each(|line| {
                    res.lock().unwrap().push_str(line);
                    res.lock().unwrap().push('\n');
                });
        });
        s.spawn(|_| {
            find(il2, ir2, ir3, content)
                .split("\n")
                .filter(|line| line.split_whitespace().any(|word| word.eq(target_str)))
                .for_each(|line| {
                    res.lock().unwrap().push_str(line);
                    res.lock().unwrap().push('\n');
                });
        });
        s.spawn(|_| {
            find(il3, ir3, ir3, content)
                .split("\n")
                .filter(|line| line.split_whitespace().any(|word| word.eq(target_str)))
                .for_each(|line| {
                    res.lock().unwrap().push_str(line);
                    res.lock().unwrap().push('\n');
                });
        });
    })
    .unwrap();

    let shmem = ShmemConf::new()
        .size(res.lock().unwrap().len() + std::mem::size_of::<usize>())
        .flink(SHMEM_FILE_PATH)
        .open()
        .unwrap();
    let mut raw_ptr = shmem.as_ptr();

    unsafe {
        let buf = &res.lock().unwrap();
        let be_u8_arr = buf.len().to_be_bytes();
        for i in 0..mem::size_of::<usize>() {
            std::ptr::write_bytes(raw_ptr, be_u8_arr[i], 1);
            raw_ptr = raw_ptr.add(1);
        }
        for i in 0..buf.len() {
            std::ptr::write_bytes(raw_ptr, (*buf.as_bytes())[i], 1);
            raw_ptr = raw_ptr.add(1);
        }
    }
    write(control_pipe_child_to_parent_write_end, &[0 as u8]).unwrap();
}
pub fn shmem_method(src_file_path: &str, dst_file_path: &str, target_str: &str) {
    open(
        SHMEM_FILE_PATH,
        OFlag::O_RDWR | OFlag::O_CREAT,
        Mode::S_IRUSR | Mode::S_IWUSR,
    )
    .unwrap();
    let control_pipe_parent_to_child = pipe().unwrap();
    let control_pipe_child_to_parent = pipe().unwrap();
    let pid = unsafe { fork() }.unwrap();
    match pid {
        Parent { child } => {
            close(control_pipe_parent_to_child.0).unwrap();
            close(control_pipe_child_to_parent.1).unwrap();
            reducer(
                src_file_path,
                dst_file_path,
                control_pipe_parent_to_child.1,
                control_pipe_child_to_parent.0,
            );

            waitpid(child, None).unwrap();
        }
        Child => {
            close(control_pipe_parent_to_child.1).unwrap();
            close(control_pipe_child_to_parent.0).unwrap();

            mapper(
                target_str,
                control_pipe_child_to_parent.1,
                control_pipe_parent_to_child.0,
            );
            exit(0);
        }
    }
}
