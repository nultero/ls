use crate::{
    files::dbg_dump_icons, 
    state::State
};

pub fn parse_args(args: &Vec<String>, stt: &mut State) {
    for arg in args {
        if arg == "--dbgicons" {
            dbg_dump_icons();

        } else if arg.starts_with("-") {
            let archars: Vec<char> = arg.chars().collect();
            for ch in &archars[1..] {
                match ch {
                    'a' => { stt.all_files = true }
                    'h' => { stt.help      = true }
                    'p' => { stt.porcelain = true }
                    _ => {
                        println!("\x1b[31munrecognized flag\x1b[0m: {ch}");
                    }
                }
            }

        } else {
            stt.has_search = true;
            stt.search_chunks.push(arg.to_string());
        }
    }
}