extern crate libc;
use std::ffi::CString;
pub fn pipe_duplex(src_file_path: &str, dst_file_path: &str, target_str: &str) {
    let filename = CString::new("demo/fifo").unwrap();
    unsafe {
        libc::mkfifo(filename.as_ptr(), libc::S_IRUSR | libc::S_IWUSR);
    }
    
    //  TODO
    panic!("Not implemented!");
}
