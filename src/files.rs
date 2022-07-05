use crate::phf::phf_map;

// this is an ugly default, what else to use?
// pub const DEF: &'static str = "⚬";

pub static FMAP: phf::Map<&'static str, &'static str> = phf_map! {
    "cfg"        => "",
    "conf"       => "",
    "css"        => "",
    "db"         => "",
    "Dockerfile" => "",
    "duck"       => "",
    "fire"       => "",
    "git"        => "",
    "go"         => "",
    "html"       => "",
    "java"       => "",
    "jpeg"       => "",
    "jpg"        => "",
    "lock"       => "",
    "mp3"        => "",
    "nix"        => "",
    "pdf"        => "",
    "png"        => "",
    "py"         => "",
    "rb"         => "",
    "rs"         => "",
    "sh"         => "",
    "txt"        => "",
    "symbol"     => " "
};

#[allow(dead_code, unused_variables)]
pub fn fmtf(fname: String) {

}