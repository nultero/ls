use phf::phf_map;
use std::{fs::DirEntry, os::unix::prelude::PermissionsExt, process::exit};

// TODO this is an ugly default, what else to use?
const DEF: &'static str = "⚬";

pub struct Ascii {
    pub icon: char,
    pub color: Option<&'static str>,
}

pub static MAP: phf::Map<&'static str, Ascii> = phf_map! {
    "cfg"        => Ascii{ icon: '', color: None },
    "conf"       => Ascii{ icon: '', color: None },
    "css"        => Ascii{ icon: '', color: None },
    "db"         => Ascii{ icon: '', color: None },
    "dir"        => Ascii{ icon: '', color: Some("\x1b[38;2;180;10;108m") },
    "Dockerfile" => Ascii{ icon: '', color: None },
    "duck"       => Ascii{ icon: '', color: None },
    "exe"        => Ascii{ icon: '', color: Some("\x1b[38;2;212;255;0m") },
    "fire"       => Ascii{ icon: '', color: None },
    "git"        => Ascii{ icon: '', color: None },
    "go"         => Ascii{ icon: '', color: None },
    "html"       => Ascii{ icon: '', color: None },
    "java"       => Ascii{ icon: '', color: None },
    "jpeg"       => Ascii{ icon: '', color: None },
    "jpg"        => Ascii{ icon: '', color: None },
    "js"         => Ascii{ icon: '', color: None },
    "lock"       => Ascii{ icon: '', color: None },
    "md"         => Ascii{ icon: '', color: None },
    "mp3"        => Ascii{ icon: '', color: None },
    "nix"        => Ascii{ icon: '', color: None },
    "pdf"        => Ascii{ icon: '', color: None },
    "png"        => Ascii{ icon: '', color: None },
    "py"         => Ascii{ icon: '', color: None },
    "rb"         => Ascii{ icon: '', color: None },
    "rs"         => Ascii{ icon: '', color: None },
    "sh"         => Ascii{ icon: '', color: None },
    "toml"       => Ascii{ icon: '', color: None },
    "txt"        => Ascii{ icon: '', color: None },
    "symbol"     => Ascii{ icon: '', color: None },
};

fn get_extension(s: &String) -> Option<&str> {
    for (idx, chr) in s.chars().rev().enumerate() {
        if chr == '.' {
            let sl = &s[s.len() - idx..]; // should grab JUST extension
            return Some(sl);
        }
    }
    return None;
}

fn fmt_default(s: &String) -> String {
    return format!("{} {}", DEF, s);
}

fn fmts(a: &Ascii, s: &String) -> String {
    if let Some(c) = a.color {
        return format!("{}{} {}\x1b[0m", c, a.icon, s);
    } else {
        return format!("{} {}", a.icon, s);
    }
}

pub fn get_pretty_version_of(de: DirEntry) -> (String, usize) {
    let tmp = de.file_name();
    let s = tmp.to_str().unwrap().to_owned();

    let res = de.metadata();
    if let Ok(md) = res {
        if md.is_dir() {
            let ascii = MAP.get("dir").unwrap();
            return (fmts(ascii, &s), s.len() + 2);
        }

        let perms = md.permissions();
        let mode = perms.mode();
        let is_exe = mode & 0o111 != 0;
        if is_exe {
            let ascii = MAP.get("exe").unwrap();
            return (fmts(ascii, &s), s.len() + 2);
        }
    }

    let mut out = String::new();
    let opt = get_extension(&s);
    match opt {
        Some(v) => {
            let opt = MAP.get(v);
            if let Some(ascii) = opt {
                out = fmts(ascii, &s);
            } else {
                out = fmt_default(&s);
            }
            return (out, s.len() + 2);
        }
        None => {}
    }

    let len = s.len() + 2;
    out = format!("{}  ", s);
    return (out, len);
}

// pub fn fmtf(fname: String, ext: &str) -> String {
//     if let Some(icon) = FMAP.get(ext) {
//         return format!("{}  {}", icon, fname);
//     } else {
//         return format!("{}  {}", DEF, fname);
//     }
// }

/// ### Debug fn
/// Used to triple-check that my fonts all draw the unicode
/// chars like I expect. Basically has to be a manual
/// test because of the visuals for now.
pub fn dbg_dump_icons() {
    for (k, v) in MAP.entries() {
        if let Some(c) = v.color {
            print!("{}{}: {}\x1b[0m\t", c, k, v.icon);
        }
        println!("{}: {}", k, v.icon);
    }
    exit(0);
}
