extern crate getopts;
use getopts::{optopt,optflag,getopts,OptGroup};
use std::os;

fn print_usage(program: &String, opts: &[OptGroup]) {
    let desc = format!(
        "Usage:\t{} [OPTION]... LAST\n\
            or:\t{} [OPTION]... FIRST LAST\n\
            or:\t{} [OPTION]... FIRST INCREMENT LAST",
            program, program, program);

    println!("{}", getopts::usage(desc.as_slice(), opts));
}

fn print_error(msg: &str) {
    println!("seq: {}", msg);
    os::set_exit_status(1);
}

fn print_seq(first: int, inc: int, last: int,
             separator: &String, equal_width: bool) {
    // If increment does not get closer to last
    if ((last - first) >= 0) != (inc >= 0) {
        return;
    }

    // Get max width (for padding)
    let get_len = |i: int| -> uint {
        return format!("{}", i).len()
    };
    let first_len = get_len(first);
    let last_len = get_len(last);
    let max_width = std::cmp::max(first_len, last_len);

    // Generate padded zeroes
    let generate_padding = |num: uint| {
        let mut padding = String::from_str("");
        for _ in range(0, num) {
            padding.push_str("0");
        }
        return padding;
    };

    // Loop
    let mut i: int = first;
    while i <= last {
        let padding =
            if !equal_width {
                String::from_str("")
            } else {
                generate_padding(max_width - get_len(i))
            };

        print!("{}{}{}", padding, i, separator);
        i = i + inc;
    }
}

fn main() {
    let args = os::args();
    let program = args.get(0).clone();

    let opts = [
        optopt("s", "separator", "use STRING to separate numbers", "STRING"),
        optflag("w", "equal-width", "equalize width by padding with leading zeroes"),
        optflag("h", "help", "display this help and exit")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f) }
    };

    let separator = match matches.opt_str("s") {
        Some(a) => a,
        None => String::from_str("\n")
    };
    let equal_width = matches.opt_present("w");
    let help = matches.opt_present("h");

    let free = matches.free.clone();
    let len = free.len();

    if help {
        print_usage(&program, opts);
    } else if len == 0 {
        print_error("missing operand");
    } else if len > 3 {
        print_error("extra operand");
    } else {
        let get_arg = |i: uint| -> Option<int> {
            from_str(free.get(i).as_slice())
        };
        let (first_opt, inc_opt, last_opt) = match len {
            1 => (Some(1), Some(1), get_arg(0)),
            2 => (get_arg(0), Some(1), get_arg(1)),
            3 => (get_arg(0), get_arg(1), get_arg(2)),
            _ => return
        };
        let (first, inc, last) = match (first_opt, inc_opt, last_opt) {
            (Some(a), Some(b), Some(c)) => (a, b, c),
            _ => {
                print_error("invalid floating point argument");
                return
            }
        };

        print_seq(first, inc, last, &separator, equal_width);
    }
}
