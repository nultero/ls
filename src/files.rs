use crate::phf::phf_map;

// this is an ugly default, what else to use?
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
    "go"         => "",
    "html"       => "",
    "java"       => "",
    "jpeg"       => "",
    "jpg"        => "",
    "js"         => "",
    "lock"       => "",
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