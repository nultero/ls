use std::{fmt::Display, fs::DirEntry, path::PathBuf};
use term_grid::Cell;

use crate::files::get_pretty_version_of;

pub struct LsStruct {
    pub search_strs: Vec<String>,
    pub ls_paths: Vec<PathBuf>,
}

#[allow(non_snake_case)]
pub fn default_Ls_struct() -> LsStruct {
    return LsStruct {
        search_strs: vec![],
        ls_paths: vec![],
    };
}

// TODO : colors
//  -> vibrant magenta for dirs
//  -> hot yellow w/ dark yellowy bg for executables
//  -> deep purple for imgs

pub fn print_red<T: Display>(s: T) {
    println!("\x1b[31m{}\x1b[0m", s);
}

pub fn get_cell_from(de: DirEntry, plain: bool) -> Cell {
    let tmp = de.file_name();
    let name = tmp.to_str().unwrap().to_owned();

    if plain {
        return Cell::from(name);
    }

    let (pretty_v, sz) = get_pretty_version_of(de);

    let mut cell = Cell::from(name);
    cell.contents = pretty_v;
    cell.width = sz;
    return cell;
}
