use std::process::exit;

use crate::phf::phf_map;

// TODO this is an ugly default, what else to use?
const DEF: &'static str = "⚬";

pub static FMAP: phf::Map<&'static str, &'static str> = phf_map! {
    "cfg"        => "",
    "conf"       => "",
    "css"        => "",
    "db"         => "",
    "dir"        => "",
    "Dockerfile" => "",
    "duck"       => "",
    "fire"       => "",
    "git"        => "",
    "go"         => "",
    "html"       => "",
    "java"       => "",
    "jpeg"       => "",
    "jpg"        => "",
    "js"         => "",
    "lock"       => "",
    "md"         => "",
    "mp3"        => "",
    "nix"        => "",
    "pdf"        => "",
    "png"        => "",
    "py"         => "",
    "rb"         => "",
    "rs"         => "",
    "sh"         => "",
    "toml"       => "",
    "txt"        => "",
    "symbol"     => " "
};

pub fn fmtf(fname: String, ext: &str) -> String {
    if let Some(icon) = FMAP.get(ext) {
        return format!("{}  {}", icon, fname);
    } else {
        return format!("{}  {}", DEF, fname);
    }
}

/// ### Debug fn
/// Used to triple-check that my fonts all draw the unicode
/// chars like I expect. Basically has to be a manual
/// test because of the visuals for now.
pub fn dbg_dump_icons() {
    for (k, v) in FMAP.entries() {
        println!("{}: {}", k, v);
    }
    exit(0);
}