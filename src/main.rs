#![feature(vec_into_raw_parts)]
mod data_structure;
mod opcode;
mod parser_atom;
fn tryvec(vec:*mut u8,a1:usize,a2:usize) -> Vec<u32> {
    unsafe {
        let mut vec:Vec<u32> = Vec::from_raw_parts(vec as *mut u32,a1,a2);
        vec.push(3);
        vec
    }
}
fn main() {
    let mut vec:Vec<u32> = vec![1,2];
    let vec_ptr = vec.into_raw_parts();
    let vec = tryvec(vec_ptr.0 as *mut u8,vec_ptr.1,vec_ptr.2);
    println!("{}",vec[2]);
}
