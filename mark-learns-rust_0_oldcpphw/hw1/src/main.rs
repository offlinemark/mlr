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
    let line = try!(get_line(stdin));
    match line.trim().parse::<T>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Parse Error".to_owned())
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

#[cfg(test)]
mod tests {
    use super::{get_input, get_line};

    #[test]
    fn gi_pos() {
        let test = "1\n";
        assert_eq!(1, get_input::<i8, &[u8]>(test.as_bytes()).unwrap());
    }

    #[test]
    fn gi_neg() {
        let test = "-1\n";
        assert_eq!(-1, get_input::<i8, &[u8]>(test.as_bytes()).unwrap());
    }

    #[test]
    fn gi_err() {
        let test = "asdf\n";
        let y =  get_input::<i8, &[u8]>(test.as_bytes());
        assert_eq!("Parse Error", y.unwrap_err());
    }

    #[test]
    fn gl() {
        let test = "asdf\n".as_bytes();
        assert_eq!("asdf", get_line(test).unwrap());
    }
}
