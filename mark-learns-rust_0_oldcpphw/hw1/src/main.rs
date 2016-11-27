use std::io;
use std::io::Write;
use std::io::Read;

fn print_greeting() {
    println!("hi welcome to the bank :)");
    println!("choose and option:");
    println!("1: open an account");
    println!("2: open mortage calculator");
    print!("> ");
    io::stdout().flush().unwrap();
}

fn get_line<R>(input: R) -> Result<String, String> where R: Read {
    let mut buf = String::new();
    for b in input.bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c == '\n' {
                    break;
                }
                buf.push(c);
            }
            Err(_) => return Err("IO Error".to_owned())
        }
    }
    Ok(buf)
 }

fn get_input<T, R>(stdin: R) -> Result<T, String> where
    T: std::str::FromStr,
    R: Read
{
    match get_line(stdin) {
        Ok(buf) => {
            match buf.trim().parse::<T>() {
                Ok(num) => Ok(num),
                Err(_) => Err("Parse Error".to_owned())
            }
        }
        Err(e) => Err(e)
    }
}

fn bank(input: i8) {
    match input {
        1 => open_acct(),
        2 => mortgage_calc(),
        _ => println!("Invalid option!")
    }
}

fn open_acct() {

}

fn mortgage_calc() {

}


fn main() {
    print_greeting();
    match get_input(io::stdin()) {
        Ok(input) => {
            bank(input);
        }
        Err(err) => {
            println!("oh no!");
            println!("{} :(", err);
        }
    }
}


