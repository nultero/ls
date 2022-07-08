extern crate phf;
extern crate rustix;

mod files;
mod pprint;

use files::FMAP;
use rustix as rx;

fn main() {

    // slightly restated winsize code example from
    // * github.com/eminence/terminal-size *
    // I am too lazy to add another dep
    let stdout = rx::io::raw_stdout();
    let file_desc: rx::fd::BorrowedFd = unsafe {
        rx::fd::BorrowedFd::borrow_raw(stdout)
    };
    
    if !rx::termios::isatty(file_desc) {
        panic!("not running with a tty"); // eh. this bin is only for me, 
    }//                                      so the panic is fine. if I run this somewhere that's on me
    
    let wsz = rx::termios::tcgetwinsize(file_desc)
                        .expect("trouble grabbing winsize from tty file handle");

    let width: u16 = wsz.ws_col;
    let height: u16 = wsz.ws_row;
    
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        pprint::pretty_print_cwd(width, height, false);
        return;
    }

    for arg in args {

        if arg == "--allicons" {
            for (k, v) in FMAP.entries() {
                println!("{}: {}", k, v);
            }
            return;
        }
    }
}
