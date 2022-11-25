mod length_of;
use length_of::lengths::{GetLengths, GetLongestItem};

pub struct Sentences<'a> {
    line: Vec<&'a str>,
}

fn main() {
    let input_vec = Sentences { line: vec![" ", "hi i'm nav"] };

    let lengths_hash = &input_vec.lengths();
    match lengths_hash {
        Ok(hash) => println!("All the items in the hashmap are: {:?}", hash),
        Err(err) => println!("There was an error: {:?}", err),
    }

    let longest_str = &input_vec.longest();
    match longest_str {
        Ok(str) => println!("[{:?}] Is the longest.", str),
        Err(err) => println!("There was an error: {:?}", err),
    }
}
