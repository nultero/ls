use crate::files::fmtf;
use crate::state::State;
use std::collections::HashSet;
use std::fs::read_dir;

/// High-level print delegator.
pub fn pretty_print_cwd(w: u16, stt: State) {
    let dir = read_dir(".").expect("problem reading current dir");

    let mut files: Vec<String> = vec![];
    let mut dirs: HashSet<String> = HashSet::with_capacity(15);

    for de in dir {
        let dirent = de.expect("problem reading direntry");
        let fname = dirent.file_name();
        if let Some(name) = fname.to_str() {
            if name.starts_with('.') {
                if !stt.all_files {
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

    files.sort_by( |a, b| 
        a.to_lowercase().cmp( &b.to_lowercase() )
    );

    let mut mx_len = 0;
    let mut file_names: Vec<String> = vec![String::new(); files.len()];

    for (i, f) in files.iter().enumerate() {
        if f.len() > mx_len {
            mx_len = f.len();
        }

        let mut has_dot = false;
        let mut idx = 0;
        for (cidx, ch) in f.chars().rev().enumerate().take(6) {
            if ch == '.' {
                has_dot = true;
                idx = cidx;
            }
        }

        if dirs.contains(f) {
            let s = fmtf(f.to_string(), &"dir");
            file_names[i] = s;
        } else if has_dot {
            let dot_idx = f.len() - idx;
            let sl = &f[dot_idx..];
            let s = fmtf(f.to_string(), sl);
            file_names[i] = s;
        } else {
            let s = fmtf(f.to_string(), "\0");
            file_names[i] = s;
        }
    }

    mx_len += 3; // pad for icons
    calc_and_print(file_names, w, mx_len);
}

#[allow(unused_variables)]
fn calc_and_print(file_names: Vec<String>, w: u16, mx_len: usize) {

    let wd = w as usize;

    // won't fit multicolumn, just print everything & quit
    if mx_len > wd {
        porcelain_print(file_names);
        return;
    }

    let mut rows: Vec<Vec<usize>> = vec!(vec!(); 1);
    let mut popped: Vec<bool> = vec![false; file_names.len()];
    let num_cols = wd / mx_len;
    let mut rwidx: usize = 0;

    for idx in 0..file_names.len() {
        if popped[idx] {
            continue;
        }

        rows[rwidx].push(idx);
        popped[idx] = true;
        
        let mut nxt = idx + num_cols;
        loop {
            if nxt >= file_names.len() {
                break;
            }

            rows[rwidx].push(nxt);
            popped[nxt] = true;
            nxt += num_cols;
        }

        rwidx += 1;
        rows.push(vec!());
    }

    let mut col_lens: Vec<usize> = vec!();
    for rw in &rows {
        for (i, idx) in rw.iter().enumerate() {
            let idx = idx.to_owned();
            if i == col_lens.len() {
                col_lens.push(0);
            }

            let sub_length = file_names[idx].len() - 2;
            if sub_length > col_lens[i] {
                col_lens[i] = sub_length + 7;
            }
        }
    }
    
    for rw in rows {
        for (i, idx) in rw.iter().enumerate() {
            let idx = idx.to_owned();
            let fname = &file_names[idx];
            let mut spaces: usize = 0;
            if fname.len() > col_lens[i] {
                println!(
                    "\nsomething went wrong at fname \x1b[31m{}\x1b[0m;\n col: {}, col_lens[i]: {}, fnamelen: {}",
                    &fname, i, col_lens[i], fname.len()
                );
            } else {
                spaces = col_lens[i] - fname.len();
            }
            print!(
                "{}{}",
                file_names[idx],
                " ".repeat(spaces),
            );
        }
        print!("\n");
    }
}

/// Zero calculation of columns.
fn porcelain_print(file_names: Vec<String>) {
    for f in file_names {
        println!("{f}");
    }
}