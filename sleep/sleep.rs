extern crate getopts;
use getopts::{optflag,getopts,OptGroup};
use std::{os};

fn print_usage(program: &String, opts: &[OptGroup]) {
    let desc = format!(
        "Usage:\t{} NUMBER[SUFFIX]\n\
        \t{} OPTION",
        program, program);

    println!("{}", getopts::usage(desc.as_slice(), opts));
}

fn print_error(err: &str) {
    println!("sleep: {}", err);
    os::set_exit_status(1);
}

fn sleep(time_str: &str) {
    let last_index = time_str.len() - 1;
    let last_char = time_str.char_at(last_index);

    let prefix_str = time_str.slice_to(last_index);

    let (duration_str, multiplier) = match last_char {
        'd' => (prefix_str, 24 * 60 * 60 * 1000),
        'h' => (prefix_str, 60 * 60 * 1000),
        'm' => (prefix_str, 60 * 1000),
        's' => (prefix_str, 1000),
        _ => (time_str, 1000)
    };

    let duration: Option<u64> = from_str(duration_str);
    let duration_u64 = match duration {
        Some(f) => f * multiplier,
        _ => {
            let error = format!("Invalid time interval: {}", time_str);
            print_error(error.as_slice());
            return;
        }
    };

    std::io::timer::sleep(duration_u64);
}

fn main() {
    let args = os::args();
    let program = args.get(0).clone();

    let opts = [
        optflag("h", "help", "display this help and exit")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f) }
    };

    let help = matches.opt_present("h");
    let free = matches.free.clone();
    let len = free.len();

    if help {
        print_usage(&program, opts);
    } else if len == 0 {
        print_error("missing operand");
    } else {
        // let a = "1.2";
        let arg = free.get(0);
        sleep(arg.as_slice());
    }
}
