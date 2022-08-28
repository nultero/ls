extern crate phf;
extern crate rustix;

mod args;
mod files;
mod pprint;
mod state;

use args::parse_args;
use rustix as rx;
use state::default_state;

fn main() {
    let mut stt = default_state();
    let args: Vec<String> = std::env::args().skip(1).collect();
    parse_args(&args, &mut stt);

    // slightly restated winsize code example from
    // * github.com/eminence/terminal-size *
    // I did not want to add another dep
    let stdout = rx::io::raw_stdout();
    let file_desc: rx::fd::BorrowedFd = unsafe {
        rx::fd::BorrowedFd::borrow_raw(stdout)
    };
    
    if !rx::termios::isatty(file_desc) {
        stt.is_tty = false;
        stt.porcelain = true;
    }

    if stt.is_tty {
        let wsz = rx::termios::tcgetwinsize(file_desc)
                            .expect("trouble grabbing winsize from tty file handle");
    
        let width: u16 = wsz.ws_col;
    
        if !stt.has_search {
            pprint::pretty_print_cwd(width, stt);
            return;
        }
    } else {
        todo!();
    }
    
}
