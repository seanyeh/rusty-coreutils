extern crate getopts;
use getopts::{optflag,getopts,OptGroup};
use std::{io,os};

static PROGRAM: &'static str = "tee";

fn print_usage(opts: &[OptGroup]) {
    let desc = format!(
        "Usage:\t{} [OPTION]... [FILE]...",
            PROGRAM);

    println!("{}", getopts::usage(desc.as_slice(), opts));
}

fn read_stdin() -> String {
    let mut s = String::from_str("");
    for line in io::stdin().lines() {
        s.push_str(line.unwrap().as_slice());
    }
    return s;
}

fn main() {
    let args = os::args();

    let opts = [
        optflag("a", "append", "append to given FILEs, do not overwrite"),
        optflag("h", "help", "display this help and exit")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f) }
    };

    let help = matches.opt_present("h");
    let free = matches.free.clone();

    if help {
        print_usage(opts);
    } else {
        let stdin_str = read_stdin();

        print!("{}", &stdin_str);

        let write_mode =
            if matches.opt_present("a") {
                std::io::Append
            } else {
                std::io::Truncate
            };

        for out_file in free.iter() {
            let path = Path::new(out_file.as_slice());
            let mut write_only = std::io::File::open_mode(&path, write_mode, std::io::Write);
            write_only.write(stdin_str.as_bytes()).unwrap();
        }
    }
}
