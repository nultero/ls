use crate::flags::Flags;
use crate::outputs;
use outputs::LsStruct;
use std::fs::{read_dir, DirEntry};

// fn do_tp() {} // recursive only

fn only_cwd(flags: &Flags) -> Vec<DirEntry> {
    let mut files: Vec<DirEntry> = vec![];
    let dir = read_dir(".").expect("read_dir failed on '.'");
    for f in dir {
        let dirent: DirEntry;
        match f {
            Ok(file) => dirent = file,
            Err(_) => continue,
        }

        let fname = dirent.file_name();
        let mut name = "";
        let name_opt = fname.to_str();
        match name_opt {
            Some(n) => name = n,
            None => {}
        }

        if name.starts_with(".") && !flags.all {
            continue;
        }

        files.push(dirent);
    }
    return files;
}

pub fn get_wd(args: LsStruct, flags: &Flags) -> Vec<DirEntry> {
    let mut files: Vec<DirEntry> = vec![];
    let n = args.ls_paths.len();
    if n == 0 {
        files = only_cwd(flags);
    }

    files.sort_by(|a, b| a.file_name().partial_cmp(&b.file_name()).unwrap());
    return files;
}
