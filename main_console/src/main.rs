use std::io::Write;
use std::io;


fn main() {
    let mut line = String::new();
    print!("Enter your name: ");
    let _ = std::io::stdout().flush();
    io::stdin().read_line(&mut line).unwrap();
    println!("Welcome: {}.", line);
}

#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn print_hi() {
        println!("Hi!");
    }

    #[test]
    fn equals_2() {
        assert_eq!(2, 2);
    }
}
