mod files;
mod flags;
mod getwd;
mod outputs;

use files::dbg_dump_icons;
use flags::Flags;
use getwd::get_wd;
use outputs::{default_Ls_struct, get_cell_from, LsStruct};
use std::{fs::DirEntry, path::Path};
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};

// TODO FILE TYPE COLORS??

//expected behavior on being passed args
// is to just *match* on those, like
// a `find` utility
//
//OPTS
// -l       perms? GNU ls compat

fn do_help() {
    println!("not impld yet");
}

fn parse_args(args: &Vec<String>, flags: &mut Flags) -> LsStruct {
    let mut output = default_Ls_struct();

    for arg in args {
        if arg.chars().nth(0) != Some('-') {
            let p = Path::new(arg);
            let opt = p.canonicalize();
            match opt {
                Ok(path_buf) => output.ls_paths.push(path_buf),
                Err(_) => output.search_strs.push(arg.clone()),
            }
            continue;
        }

        for c in arg.chars() {
            match c {
                'g' => dbg_dump_icons(),
                'h' => flags.help = true,
                'r' => flags.recent = true,
                'R' => flags.recurse = true,
                'p' => flags.plain = true,
                '1' | 'o' => flags.oneline = true,
                '-' => {}
                _ => {
                    outputs::print_red(format!("unrecognized arg: {}", c));
                }
            }
        }
    }

    if flags.help {
        do_help();
        std::process::exit(0);
    }

    return output;
}

fn get_grid(files: Vec<DirEntry>, flags: &Flags) -> Grid {
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(2),
        direction: Direction::TopToBottom,
    });

    let plain = flags.plain;
    for f in files {
        let cell = get_cell_from(f, plain);
        grid.add(cell);
    }
    return grid;
}

fn main() {
    let mut flags = Flags {
        recent: false,
        plain: false,
        recurse: false,
        all: false,
        oneline: false,
        help: false,
    };
    if !isatty::stdout_isatty() {
        flags.plain = true;
        flags.oneline = true;
    }

    let mut term_width: usize;
    match term_size::dimensions_stdout() {
        Some(t) => term_width = t.0,
        None => {
            println!("term_size failed, but isatty succeeded somehow");
            std::process::exit(1);
        }
    }

    let args: Vec<String> = std::env::args().skip(1).collect();
    let ls_vals = parse_args(&args, &mut flags);

    let files = get_wd(ls_vals, &flags);

    let grid = get_grid(files, &flags);

    while term_width > 1 {
        let disp_opt = grid.fit_into_width(term_width);
        match disp_opt {
            Some(display) => {
                println!("{}", display);
                break;
            }
            None => {}
        }
        term_width -= 1;
    }
}
