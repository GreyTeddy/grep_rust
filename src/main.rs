use std::{env, ffi::OsString};


fn main() {
    let mut iterator = env::args_os().skip(1);

    let query = iterator.next().expect("No filename or query specified"); 
    let file_path = iterator.next().expect("No filename specified");

    
    println!("{}",format!("Searching for {:?}\nIn file {:?}",query,file_path));
}