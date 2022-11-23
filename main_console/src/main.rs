use std::io::Write;
use std::io;
mod length_of;
mod print_numbers;
use length_of::return_values::return_okay;
use length_of::return_values::return_hello;
use print_numbers::print_number;


#[allow(dead_code)]
fn input_name(){
    let mut line = String::new();
    print!("Enter your name: ");
    let _ = std::io::stdout().flush();
    io::stdin().read_line(&mut line).unwrap();
    println!("Welcome: {}.", line);
}

fn main() {
    println!("Return values says: {}", return_hello());
    print_number(8);
    println!("Return values says: {}", return_okay());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_hi() {
        println!("Hi!");
    }

    #[test]
    fn equals_2() {
        assert_eq!(2, 2);
    }

    #[test]
    fn printed_hello(){
        assert_eq!(return_hello(), String::from("Hello world!"))
    }
}
