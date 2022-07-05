
#[allow(unused_imports)]
use crate::files::fmtf;
use std::fs::read_dir;


pub fn pretty_print_cwd(w: u16, h: u16) {
    let dir = read_dir(".").expect("problem reading current dir");
    let mut files: Vec<String> = vec!();
    for de in dir {
        let dirent = de.expect("problem reading direntry");
        let fname = dirent.file_name();
        if let Some(name) = fname.to_str() {
            files.push(name.to_string());
        }
    }

    // have to parse all of these and colorize the output better
    println!("{:?}", files);

    // have to compute mid distance with the term size
    // but for now just printing it to visualize some common
    // term windows I use size-wise
    println!("w: {}, h: {}", w, h);
}