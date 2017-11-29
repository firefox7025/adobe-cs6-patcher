extern crate glob;
extern crate crypto;

use glob::glob;


use std::io::Read;


use crypto::md5::Md5;
use crypto::digest::Digest;

use std::fs::File;

use std::path::PathBuf;
use std::result::Result;


fn main() {
    println!("Finding amtlib.dll files");
    let files = find_core_files();
    println!("Found {:?} files to patch", files.len());
    for x in files {
        println!("{:?}", x);
        hash_files(x);
    }
}


fn hash_files(f: PathBuf){

    let f = File::open(f.as_path()).expect("Unable to open");
    let x: Vec<_> = f.bytes().filter_map(Result::ok).collect();

    println!("{:?}", x);



//    return hash;
}



fn find_core_files() -> Vec<PathBuf> {
    let paths: Vec<_> =  glob("C:/**/amtlib.dll").unwrap().filter_map(Result::ok).collect();
    return paths;
}
