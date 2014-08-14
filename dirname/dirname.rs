extern crate getopts;
use getopts::{optflag,getopts,OptGroup};
use std::os;

static PROGRAM: &'static str = "dirname";

fn print_usage(opts: &[OptGroup]) {
    let desc = format!(
        "Usage:\t{} [OPTION] NAME\n\
        Output each NAME with its last non-slash component and trailing slashes\n\
        removed; if NAME contains no /'s, output '.' \
        (meaning the current directory).",
            PROGRAM);

    println!("{}", getopts::usage(desc.as_slice(), opts));
}

fn print_error(err: &str) {
    println!("{}: {}", PROGRAM, err);
    os::set_exit_status(1);
}

fn get_slash_index(s: &String) -> uint {
    let sliced = s.as_slice();
    let mut is_last = true;
    for (i, c) in sliced.chars().rev().enumerate() {
        if c == '/' && !is_last {
            return s.len() - i;
        } else if c != '/' {
            is_last = false;
        }
    }
    return 0;
}

fn do_dirname(s: &String, newline: &str) {
    let index = get_slash_index(s);
    if index == 0 {
        print!(".{}", newline);
    } else {
        print!("{}{}", s.as_slice().slice_to(index - 1), newline);
    }
}

fn main() {
    let args = os::args();

    let opts = [
        optflag("z", "zero", "end each output line with NUL, not newline"),
        optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f) }
    };

    let help = matches.opt_present("h");
    let newline = if matches.opt_present("z") { "" } else { "\n" };

    let free = matches.free;
    let len = free.len();

    if help {
        print_usage(opts);
    } else if len == 0 {
        print_error("missing operand");
    } else {
        for s in free.iter() {
            do_dirname(s, newline);
        }
    }
}
