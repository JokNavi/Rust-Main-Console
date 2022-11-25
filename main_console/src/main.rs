mod length_of;
mod files;
use length_of::lengths::{GetLengths, GetLongestItem};
use files::file_handler::read_file_vec;


pub struct Sentences {
    line: Vec<String>,
}

fn main() {
    
    //Create an input vector of type Sentences similar to the one from a read file.
    let input_vec = Sentences { line: [" ", "Hi i'm nav!"].map(String::from).to_vec()};
    
    //Print all lengths of the contents in the previous vector as a hashmap.
    println!("All the items in the hashmap are: {:?}", &input_vec.lengths());
    
    //Print the longest item in the Vector
    println!("[{:?}] Is the longest.", &input_vec.longest());
    
    println!("{:?}", read_file_vec("src/files/Input.txt"));

}
