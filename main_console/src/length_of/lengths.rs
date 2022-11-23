use std::io::Write;
use std::{io, vec}; //needed for flushing

struct Sentences<'a>(Vec<&'a str>);

trait GetLengths {
    fn lengths(&self) -> Vec<usize>;
}

impl<'a> GetLengths for Sentences<'a> {
    fn lengths(&self) -> Vec<usize> {
        self.0.iter().map(|x| x.len()).collect::<Vec<usize>>()
    }
}

trait GetLongestItem {
    fn longest(&self) -> &str;
}

impl<'a> GetLongestItem for Sentences<'a> {
    fn longest(&self) -> &str {
        self.0.iter().fold(
            self.0[0],
            |acc, &item| {
                if item.len() > acc.len() {
                    item
                } else {
                    acc
                }
            },
        )
    }
}

pub fn print_length() {
    let input_array = Sentences(vec!["a", "bb", "ccc", "hi i'm nav"]);
    println!("\"{}\" is the longest sentence.", input_array.longest());
    println!("{:?}", input_array.lengths());
}

#[allow(dead_code)]
pub fn print_str_length() {
    print!("Please input your sentence: "); //print!() so we can take input on the same line.
    let _ = io::stdout().flush(); //Flushes so program compiles.
    let user_input = {
        let mut nth_term = String::new();
        io::stdin().read_line(&mut nth_term).expect("I/O error");
        nth_term
    };
    println!(
        "{} Is {} characters long.",
        user_input.trim(),
        user_input.trim().len()
    ) //Removes the trailing newline for accurate length calculations.
}
