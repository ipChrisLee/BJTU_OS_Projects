mod connect_duplex_pipe;
mod connect_with_pipe;
mod connect_with_shmem;
mod connect_with_socket;

use crate::connect_duplex_pipe::pipe_duplex;
use crate::connect_with_pipe::{pipe_half_duplex, pipe_navie, pipe_with_big_buffer};
use crate::connect_with_shmem::shmem_method;
use crate::connect_with_socket::socket_method;
use core::panic;
use log4rs;
use nix::fcntl::{open, OFlag};
use nix::sys::socket;
use nix::sys::stat::Mode;
use nix::sys::wait::{wait, waitpid};
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{close, dup2, fork, pipe, write};
use std::env;
use std::env::{args, Args};
use std::io::stdin;
use std::process::exit;
use std::time::Instant;

fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    log4rs::init_file("log4rs_ipc_practice.yaml", Default::default()).unwrap();
    let args: Vec<_> = args().collect();
    let src_file_path = args[1].as_str();
    let dst_file_path = args[2].as_str();
    let target_str = args[3].as_str();
    let task_name = args[4].trim();

    let now = Instant::now();
    match task_name {
        "pipe_navie" => {
            //  pipe
            pipe_navie(src_file_path, dst_file_path, target_str);
        }
        "pipe_with_big_buffer" => {
            pipe_with_big_buffer(src_file_path, dst_file_path, target_str);
        }
        "pipe_half_duplex" => {
            pipe_half_duplex(src_file_path, dst_file_path, target_str);
        }
        "pipe_duplex" => {
            pipe_duplex(src_file_path, dst_file_path, target_str);
        }
        "socket_method" => {
            socket_method(src_file_path, dst_file_path, target_str);
        }
        "shmem_method" => {
            shmem_method(src_file_path, dst_file_path, target_str);
        }
        _ => {
            panic!("Error task_name={}", task_name);
        }
    }
    let elapsed = now.elapsed();
    println!("Time cost:{:.2?}", elapsed);
}
