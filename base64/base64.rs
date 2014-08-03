extern crate getopts;
use getopts::{optflag,getopts,OptGroup};
use std::{io,os};

fn print_usage(program: &String, opts: &[OptGroup]) {
    let desc = format!(
        "Usage:\t{} [OPTION]... [FILE]...",
            program);

    println!("{}", getopts::usage(desc.as_slice(), opts));
}

fn print_error(program: &String, err: &str) {
    println!("{}: {}", program, err);
    os::set_exit_status(1);
}

fn read_stdin() -> String {
    let mut s = String::from_str("");
    for line in io::stdin().lines() {
        s.push_str(line.unwrap().as_slice());
    }
    return s;
}


// char as u8 -> binary string
fn to_binary_string(b: u8, len: uint) -> String {
    let mut s = String::from_str("");
    let mut i = b;

    for _ in range(0u, len) {
        let cur = i%2;
        i /= 2;

        s = format!("{}{}", cur, s);
    }
    s
}

fn encode_byte(b: u8) -> u8 {
    if b <= 25 {
        b + 65
    } else if b <= 51 {
        b + (97 - 26)
    } else if b <= 61 {
        b - 4
    } else if b == 62 {
        43
    } else {
        // if b == 63
        47
    }
}

enum InputErr { InvalidCharacter }
fn decode_byte(b: u8) -> Result<u8, InputErr> {
    if b >= 65 && b <= 90 {
        Ok(b - 65)
    } else if b >= 97 && b <= 122 {
        Ok((b - 97) + 26)
    } else if b >= 48 && b <= 57 {
        Ok((b - 48) + 52)
    } else if b == 43 {
        Ok(62)
    } else if b == 47 {
        Ok(63)
    } else {
        Err(InvalidCharacter)
    }
}

fn binary_to_chars(binary_chars: Vec<u8>, len: uint, f: |u:u8| -> u8) -> String {
    let mut result = String::from_str("");
    let mut pow = 1;
    let mut buf = 0;
    for (i, digit) in binary_chars.iter().rev().enumerate() {
        let addend = (digit - 48) * pow;
        buf += addend;
        pow *= 2;

        if i%len == len - 1 {
            let c = f(buf) as char;
            result = format!("{}{}", c, result);

            // Reset
            pow = 1;
            buf = 0;
        }
    }
    result
}

fn encode_base64(original_str: String) -> String {
    let bytes = original_str.clone().into_bytes();

    // Create u8 vec -> binary string
    let mut s = String::from_str("");

    for b in bytes.iter() {
        let cur = to_binary_string(*b, 8);
        s = s + cur;
    }

    // Add extra 0's if not divisible by 6
    let mut binary_chars = s.into_bytes();
    for _ in range(0u, (6 - binary_chars.len()%6)%6) {
        binary_chars.push(48);
    }

    let result = binary_to_chars(binary_chars, 6, encode_byte);

    // Add padding
    let padding = match original_str.len()%3 {
        0 => "",
        1 => "==",
        _ => "="
    };
    format!("{}{}", result, padding)
}

fn decode_base64(original_str: String) -> Result<String, InputErr> {
    let bytes = original_str.clone().into_bytes();

    // Create u8 vec -> binary string
    let mut s = String::from_str("");

    for b in bytes.iter() {
        let val = match decode_byte(*b) {
            Ok(x) => x,
            Err(e) => return Err(e)
        };

        let cur = to_binary_string(val, 6);
        s = s + cur;
    }

    // Add extra 0's if not divisible by 8
    let mut binary_chars = s.clone().into_bytes();
    for _ in range(0u, (8 - binary_chars.len()%8)%8) {
        binary_chars.push(48);
    }

    let result = binary_to_chars(binary_chars, 8, |u:u8| -> u8 { u });

    Ok(result)
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
        optflag("d", "decode", "decode data"),
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
        return
    } else if len > 1 {
        print_error(&program, "extra operand");
        return
    }

    let s =
        if len == 0 || (*free.get(0)) == String::from_str("-") {
            read_stdin()
        } else {
            match read_file(free.get(0)) {
                Ok(s) => s,
                Err(_) => {
                    print_error(&program, "No such file or directory");
                    return
                }

            }
        };

    if matches.opt_present("d") {
        match decode_base64(s) {
            Ok(s) => println!("{}", s),
            Err(_) => print_error(&program, "Input error")
        }
    } else {
        println!("{}", encode_base64(s));
    }
}
