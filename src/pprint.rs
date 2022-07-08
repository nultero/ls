
use crate::files::fmtf;
use std::fs::read_dir;
use std::collections::HashSet;

pub fn pretty_print_cwd(w: u16, h: u16, flg_all: bool) {
    let dir = read_dir(".").expect("problem reading current dir");

    let mut files: Vec<String> = vec!();
    let mut dirs: HashSet<String> = HashSet::with_capacity(15);

    for de in dir {
        let dirent = de.expect("problem reading direntry");
        let fname = dirent.file_name();
        if let Some(name) = fname.to_str() {

            if name.starts_with('.') {
                if !flg_all {
                    continue;
                }
            }

            if let Ok(md) = dirent.metadata() {
                if md.is_dir() {
                    dirs.insert(name.to_string());
                }
            }

            files.push(name.to_string());
        }
    }

    files.sort();

    let mut mx_len = 0;
    let mut file_names: Vec<String> = vec![String::new(); files.len()];

    for (i, f) in files.iter().enumerate() {
        if f.len() > mx_len {
            mx_len = f.len();
        }

        let mut has_dot = false;
        let mut idx = 0;
        for (ci, ch) in f.chars().rev().enumerate().take(6) {
            if ch == '.' {
                has_dot = true;
                idx = ci;
            }
        }

        if dirs.contains(f) {
            let s = fmtf(f.to_string(), &"dir");
            file_names[i] = s;
            // file_names.push(s);
            
        } else if has_dot {
            let dot_idx = f.len() - idx;
            let sl = &f[dot_idx..];
            let s = fmtf(f.to_string(), sl);
            file_names[i] = s;
            // file_names.push(s);
            
        } else {
            let s = fmtf(f.to_string(), "zzzzz");
            file_names[i] = s;
            // file_names.push(s);
        }
    }
    
    mx_len += 2; // pad for icons

    for f in file_names {
        print!("{} \n", f);
    }
    print!("\n");

    // have to compute mid distance with the term size
    // but for now just printing it to visualize some common
    // term windows I use size-wise
    println!("w: {}, h: {}, mxlen: {}", w, h, mx_len);
}