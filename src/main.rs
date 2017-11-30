extern crate glob;
extern crate blake2;

use glob::glob;
use std::io::Write;
use blake2::{Blake2b, Digest};
use std::fs::File;
use std::path::PathBuf;
use std::result::Result;


fn main() {
    println!("Finding amtlib.dll files");
    let files = find_core_files();
    println!("Found {:?} files to patch", files.len());
    for x in files {
        let hash = hash_files(&x);
        replace_old_files(&hash, &x);
    }
}

//
// The stored values in this method has the md5 hashes from all the established core files that need replaced.
//
fn replace_old_files(hash: &String, path: &PathBuf) {

    let x86_bytes = include_bytes!("32-bit/amtlib.dll");
    let x64_bytes = include_bytes!("64-bit/amtlib.dll");

    let hash = hash.as_str();

    let x86_files = vec![
        "12edb238aaaa50168b58e5684f4ffec388575d4a4cf007c46edc0d304c055ab94e40225b920d9c791669da7bd67a147a5117de0dff8b365c236457c427a81bb5",
        "44cf6af23abe1eabcde29a8c53baef4361e3fc7492bc249019ea8426358d4741994d7789f625b3503b7f57ab628318befffc5a11d6442f75be1a3e80a46337e9",
        "54f69efd224dd338309a1a6aa09116b7ea3f907353020a832546c50d482e5e6e176cf1cfd06f13486db732e82b9c8274a2387b03d2bb3d4beac9cb2d00e93e5a"];
    let x64_files = vec![
        "017e4d759e36931f79c7eb61aafad48ef3b24cf852d1552ddf715ecf4459e86ec0c6aee21a995084976b3cd9223994cbe4a5379de2864b4e8b15ecdaa59a0a0b",
        "b2f33bde6d5a3f284e81b5c1819627bd04cc81d5ccec32f3c10ae34e4ec058ed3ddb5beb90fcd3469d4614609c9fb2c275d4babd6b7b7a5e1b4ea1bd7a5e0f8c"];

    if x86_files.contains(&hash) {
        let p = path.as_path();
        std::fs::remove_file(&p).expect("Unable to remove a file");
        let mut cracked_file = File::create(&p).expect("Unable to create cracked file, a reinstall will now be required.");
        cracked_file.write_all(x86_bytes).expect("Writing the bytes has failed. A reinstall is now going to be required.");
        println!("x86 file used {:?}", &p);
    }

    else if x64_files.contains(&hash) {
        let p = path.as_path();
        std::fs::remove_file(&p).expect("Unable to remove a file");
        let mut cracked_file = File::create(&p).expect("Unable to create cracked file, a reinstall will now be required.");
        cracked_file.write_all(x64_bytes).expect("Writing the bytes has failed. A reinstall is now going to be required.");
        println!("x64 file used {:?}", p);
    }
}

fn hash_files(f: &PathBuf) -> String {
    let path = f.as_path();
    let mut file = File::open(&path).expect("Unable to open");;
    let hash = Blake2b::digest_reader(&mut file).expect("Unable to read file");

    let hash_code = format!("{:x}", hash);

    return hash_code;
}


fn find_core_files() -> Vec<PathBuf> {
    let paths: Vec<_> =  glob("C:/**/amtlib.dll").unwrap().filter_map(Result::ok).collect();
    return paths;
}
