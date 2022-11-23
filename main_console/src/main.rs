use std::io;
use std::io::Write;
mod length_of;
use length_of::print_lengths::print_str_length;

fn main() {
    print!("Please input your sentence: ");
    let _ = io::stdout().flush();
    let user_input = {
        let mut nth_term = String::new();
        io::stdin().read_line(&mut nth_term).expect("I/O error");
        nth_term
    };

    //let user_input = "Hi!".to_string();
    print_str_length(user_input);
}


#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn equals_2() {
        assert_eq!(2, 2);
    }
}
