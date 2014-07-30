extern crate getopts;
extern crate debug;
use getopts::{optflag,getopts,OptGroup};
use std::{io,os};

fn print_usage(program: &String, opts: &[OptGroup]) {
    let desc = format!(
        "Usage:\t{} [OPTION]... [FILE]...",
            program);

    println!("{}", getopts::usage(desc.as_slice(), opts));
}

fn print_file_error(filename: &str) {
    println!("wc: {}: No such file or directory", filename);
    os::set_exit_status(1);
}

struct WcInfo {
    lines: Option<uint>,
    words: Option<uint>,
    chars: Option<uint>,
    bytes: Option<uint>,
    max_l: Option<uint>
}

impl std::fmt::Show for WcInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}{}{}",
               option_str(self.lines),
               option_str(self.words),
               option_str(self.chars),
               option_str(self.bytes),
               option_str(self.max_l))
    }
}

impl Add<WcInfo, WcInfo> for WcInfo {
    fn add(&self, rhs: &WcInfo) -> WcInfo {
        WcInfo {
            lines: option_sum(self.lines, rhs.lines),
            words: option_sum(self.words, rhs.words),
            chars: option_sum(self.chars, rhs.chars),
            bytes: option_sum(self.bytes, rhs.bytes),
            max_l: option_max(self.max_l, rhs.max_l),
        }
  }
}

struct Config {
    lines: bool,
    words: bool,
    chars: bool,
    bytes: bool,
    max_l: bool
}

fn get_lines(s: &String) -> uint {
    let mut newlines = 0;
    for c in s.as_slice().chars() {
        if c == '\n' { newlines += 1; }
    };
    newlines
}

fn is_whitespace(c: char) -> bool {
    match c {
        '\n' | '\t' | ' ' => true,
        _ => {
            let c_ascii = c as uint;
            (c_ascii >= 11 && c_ascii <= 13)
        }
    }
}

fn get_words(s: &String) -> uint {
    let mut words = 0;
    let mut has_words = false;
    for c in s.as_slice().chars() {
        let cur_ws = is_whitespace(c);

        if has_words && cur_ws {
            words += 1;
        }
        has_words = !cur_ws;
    }
    words
}

fn get_chars(s: &String) -> uint {
    s.as_slice().char_len()
}

fn get_bytes(s: &String) -> uint {
    s.clone().into_bytes().len()
}

fn get_max_l(s: &String) -> uint {
    let mut max_l: uint = 0;
    let mut cur_length = 0;
    for c in s.as_slice().chars() {
        if c != '\n' {
            cur_length += 1;
            max_l = std::cmp::max(cur_length, max_l);
        } else {
            cur_length = 0;
        }
    }
    max_l as uint
}

fn option_sum(a: Option<uint>, b: Option<uint>) -> Option<uint> {
    match (a, b) {
        (Some(i1), Some(i2)) => Some(i1 + i2),
        _ => None
    }
}

fn option_max(a: Option<uint>, b: Option<uint>) -> Option<uint> {
    match (a, b) {
        (Some(i1), Some(i2)) => Some(std::cmp::max(i1, i2)),
        _ => None
    }
}

fn option_str(a: Option<uint>) -> String {
    match a {
        Some(i) => format!("\t{}", i),
        _ => String::from_str("")
    }
}

fn get_wc(s: &String, conf: Config) -> WcInfo {
    let lines = if conf.lines { Some(get_lines(s)) } else { None };
    let words = if conf.words { Some(get_words(s)) } else { None };
    let chars = if conf.chars { Some(get_chars(s)) } else { None };
    let bytes = if conf.bytes { Some(get_bytes(s)) } else { None };
    let max_l = if conf.max_l { Some(get_max_l(s)) } else { None };

    WcInfo {
        lines: lines,
        words: words,
        chars: chars,
        bytes: bytes,
        max_l: max_l
    }
}

fn do_wc(strings: Vec<(String,&String)>, conf: Config) {
    let mut num = 0;

    let mut sum = WcInfo {
        lines: Some(0),
        words: Some(0),
        chars: Some(0),
        bytes: Some(0),
        max_l: Some(0)
    };
    for tup in strings.iter() {
        let (ref s, name) = *tup;
        let suffix =
            if name.as_slice() != "" {
                format!("\t{}", name)
            } else {
                String::from_str("")
            };
        let wc = get_wc(s, conf);
        println!("{}{}", wc, suffix);

        sum = wc + sum;
        num += 1;
    }
    if num > 1 {
        println!("{}\t{}", sum, "total");
    }
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

fn main() {
    let args = os::args();
    let program = args.get(0).clone();

    let opts = [
        optflag("l", "lines", "print the newline counts"),
        optflag("w", "words", "print the word counts"),
        optflag("m", "chars", "print the character counts"),
        optflag("c", "bytes", "print the byte counts"),
        optflag("L", "max-length", "print the length of the longest line"),
        optflag("h", "help", "display this help and exit")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f) }
    };

    let opt_l = matches.opt_present("l");
    let opt_w = matches.opt_present("w");
    let opt_m = matches.opt_present("m");
    let opt_c = matches.opt_present("c");
    let opt_L = matches.opt_present("L");

    // If no flags, then go with default (-l -w -c)
    let noflags = !(opt_l || opt_w || opt_m || opt_c || opt_L);
    let config = Config {
        lines: if noflags { true } else { opt_l },
        words: if noflags { true } else { opt_w },
        chars: opt_m,
        bytes: if noflags { true } else { opt_c },
        max_l: opt_L
    };

    let help = matches.opt_present("h");
    let free = matches.free.clone();
    let len = free.len();

    if help {
        print_usage(&program, opts);
        return;
    }

    let s;
    let blank = String::from_str("");
    let strings: Vec<(String, &String)> =
        if len == 0 {
            s = read_stdin();
            vec![(s, &blank)]
        } else {
            // Read from files
            let mut v = vec![];
            for filename in free.iter() {
                let temp = read_file(filename);
                if !temp.is_ok() {
                    print_file_error(filename.as_slice());
                    return;
                }
                v.push((temp.unwrap(), filename));
            }
            v
        };

    do_wc(strings, config);
}
