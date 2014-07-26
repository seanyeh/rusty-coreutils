use std::os;

fn main() {
    let args = os::args();
    let len = args.len();
    let default_yes = String::from_str("y");

    let yes = match len {
        1 => default_yes,
        _ => args.slice(1, len).connect(" ")
    };

    if yes.as_slice().char_at(0) == '-' {
        if yes == String::from_str("--help") {
            println!("Help");
        } else {
            println!("Unrecognized option: {}", yes);
        }

    } else{
        loop{
            println!("{}", yes);
        }
    }
}
