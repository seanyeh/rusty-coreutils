extern crate getopts;
use getopts::{optopt,optflag,getopts,OptGroup};
use std::os;

fn print_usage(program: &String, opts: &[OptGroup]) {
    let desc = format!(
        "Usage:\t{} NAME [SUFFIX]\n\
            or:\t{} OPTION... NAME...",
            program, program);

    println!("{}", getopts::usage(desc.as_slice(), opts));
}

fn print_error(msg: &str) {
    println!("basename: {}", msg);
    os::set_exit_status(1);
}

fn get_slash_index(s: &String) -> uint {
    let sliced = s.as_slice();
    for (i, c) in sliced.chars().rev().enumerate() {
        if c == '/' {
            return s.len() - i;
        }
    }
    return 0;
}

fn print_basenames(v: &Vec<String>, suffix: &String, newline: &str) {
    let suffix_len = suffix.len();
    for word in v.iter() {
        let index = get_slash_index(word);
        let sliced = word.as_slice().slice_from(index);

        let match_index = sliced.len() - suffix_len;
        let match_str = sliced.slice_from(match_index);
        let result =
            if match_str == suffix.as_slice() {
                sliced.slice_to(match_index)
            } else {
                sliced
            };
        print!("{}{}", result, newline);
    }
}

fn main() {
    let args = os::args();
    let program = args.get(0).clone();

    let opts = [
        optopt("s", "suffix", "remove a trailing suffix", "SUFFIX"),
        optflag("a", "multiple", "support multiple arguments and treat each as a NAME"),
        optflag("z", "zero", "end each output line with NUL, not newline"),
        optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f) }
    };

    let help = matches.opt_present("h");
    let suffix = matches.opt_str("s");
    let multiple = matches.opt_present("a") || matches.opt_present("s");
    let newline = if matches.opt_present("z") { "" } else { "\n" };

    let mut free = matches.free.clone();
    let len = free.len();

    if help {
        print_usage(&program, opts);
    } else if len == 0 {
        print_error("missing operand");
    } else if len > 2 && !multiple {
        print_error("too many operands");
    } else {
        let suffix_str = match suffix {
            Some(a) => a,
            None =>
                if len < 2 || multiple {
                    String::from_str("")
                } else {
                    // If two args, the second is the suffix
                    let s = (*free.get(1)).clone();
                    free.pop();
                    s
                }
        };

        print_basenames(&free, &suffix_str, newline);
    }
}
