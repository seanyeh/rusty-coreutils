extern crate getopts;
use getopts::{optflag,getopts,OptGroup,optopt};
use std::{io,os};

fn print_usage(program: &String, opts: &[OptGroup]) {
    let desc = format!(
        "Usage:\t{} [OPTION]... FILE1 FILE2\n\n\
        Compare sorted files FILE1 and FILE2 line by line.\n\n\
        With no options, produce three-column output.\n\
        Column one contains lines unique to FILE1,\
        column two contains lines unique to FILE2,\n\
        and column three contains lines common to both files.\n\n\
        If FILE is '-', read from stdin.",
            program);

    println!("{}", getopts::usage(desc.as_slice(), opts));
}

fn print_error(error: &str) {
    println!("comm: {}", error);
    os::set_exit_status(1);
}

fn print_file_error(filename: &String) {
    print_error(format!("{}: No such file or directory", filename).as_slice());
}

fn read_stdin() -> String {
    let mut s = String::from_str("");
    for line in io::stdin().lines() {
        s.push_str(line.unwrap().as_slice());
    }
    return s;
}

#[deriving(Show)]
enum FileErr { FileNotFound }
fn read_file(filename: &String) -> Result<String, FileErr> {
    let filename_s = filename.as_slice();
    if filename_s == "-" {
        Ok(read_stdin())
    } else{
        let path = Path::new(filename.as_slice());
        let mut file = std::io::File::open(&path);
        let temp = file.read_to_end();
        match temp {
            Ok(s) => {
                let contents = std::str::from_utf8_owned(s);
                Ok(contents.unwrap())
            },
            Err(_) => {
                Err(FileNotFound)
            }
        }
    }
}

struct CommConf {
    show1: bool,
    show2: bool,
    show3: bool,
    delimiter: String,
    ignore_error: bool,
    exit_on_error: bool
}

fn print_col(s: &str, col: uint, conf: &CommConf){
    let delimiter = conf.delimiter.clone();
    let blank_str = String::from_str("");
    let tab1 = if conf.show1 { delimiter.clone() } else { blank_str.clone() };
    let tab2 = if conf.show2 { delimiter.clone() } else { blank_str.clone() };

    match col {
        1 => if conf.show1 { println!{"{}", s}; },
        2 => if conf.show2 { println!{"{}{}", tab1, s}; },
        _ => if conf.show3 { println!{"{}{}{}", tab1, tab2, s}; }
    }
}

fn check_order(s1: &str, s2: &str, filenum: uint, conf: &CommConf) -> bool {
    if conf.ignore_error { return true; }
    if s2 != "" {
        if s1 < s2 {
            print_error(format!("file {} is not in sorted order", filenum).as_slice());
            return !conf.exit_on_error;
        }
    }
    return true;
}

fn do_comm(string1: String, string2: String, conf: &CommConf) {
    let mut s_it1 = string1.as_slice().split('\n');
    let mut s_it2 = string2.as_slice().split('\n');

    let mut s_op1 = s_it1.next();
    let mut s_op2 = s_it2.next();

    let mut prev_s1 = "";
    let mut prev_s2 = "";

    while !s_op1.is_none() || !s_op2.is_none() {
        let s1;
        let s2;
        if s_op1.is_none() {
            s2 = s_op2.unwrap();

            // Check order
            if !check_order(s2, prev_s2, 2, conf) { return; }
            prev_s2 = s2.clone();

            print_col(s2, 2, conf);
            s_op2 = s_it2.next();
        } else if s_op2.is_none() {
            s1 = s_op1.unwrap();

            // Check order
            if !check_order(s1, prev_s1, 1, conf) { return; }
            prev_s1 = s1.clone();

            print_col(s1, 1, conf);
            s_op1 = s_it1.next();

        } else {
            s1 = s_op1.unwrap();
            s2 = s_op2.unwrap();

            // Check order
            if !check_order(s1, prev_s1, 1, conf) { return; }
            prev_s1 = s1.clone();
            if !check_order(s2, prev_s2, 2, conf) { return; }
            prev_s2 = s2.clone();

            // if the same, print in the middle
            if s1 == s2 {
                print_col(s1, 3, conf);
                s_op1 = s_it1.next();
                s_op2 = s_it2.next();
            } else if s1 < s2 {
                print_col(s1, 1, conf);
                s_op1 = s_it1.next();
            } else {
                print_col(s2, 2, conf);
                s_op2 = s_it2.next();
            }
        }
    }
}

fn main() {
    let args = os::args();
    let program = args.get(0).clone();

    let opts = [
        optflag("1", "", "suppress column 1 (lines unique to FILE1)"),
        optflag("2", "", "suppress column 2 (lines unique to FILE2)"),
        optflag("3", "", "suppress column 3 (lines that appear in both files)"),
        optflag("", "nocheck-order", "do not check that the input is correctly sorted"),
        optflag("", "check-order",
                "check that the input is correctly sorted, even if all input lines are pairable"),
        optopt("", "output-delimiter", "separate columns with STR", "STR"),
        optflag("h", "help", "display this help and exit")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f) }
    };

    let help = matches.opt_present("h");
    let free = matches.free.clone();
    let len = free.len();

    let tab_str = String::from_str("\t");
    let conf =
        CommConf {
            show1: true && !matches.opt_present("1"),
            show2: true && !matches.opt_present("2"),
            show3: true && !matches.opt_present("3"),
            delimiter: match matches.opt_str("output-delimiter") {
                Some(s) => s,
                None => tab_str
            },
            ignore_error: false || matches.opt_present("nocheck-order"),
            exit_on_error: true && matches.opt_present("check-order")
        };

    if help {
        print_usage(&program, opts);
        return
    }
    match len {
        0 | 1 => print_error("missing operand"),
        2 => {
            let f1 = free.get(0);
            let f2 = free.get(1);

            let f1_s = f1.as_slice();
            let f2_s = f2.as_slice();

            if f1_s == "-" && f2_s == "-" {
                print_error("Both files cannot be '-'");
                return
            }

            match (read_file(f1), read_file(f2)) {
                (Ok(s1), Ok(s2)) =>
                    // Files are valid: do comm
                    do_comm(s1, s2, &conf),
                (Err(_), _) => {
                    print_file_error(f1);
                },
                (_, Err(_)) => {
                    print_file_error(f2);
                }
            };
        }
        _ => {
            let s = format!("extra operand: \'{}\'", free.get(2));
            print_error(s.as_slice());
        }
    };
}
