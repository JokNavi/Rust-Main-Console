mod length_of;
mod files;
use length_of::lengths::{GetLengths, GetLongestItem};
use files::file_handler::read_file_vec;


pub struct Sentences {
    line: Vec<String>,
}

fn main() {

    /*
    //Create an input vector of type Sentences similar to the one from a read file.
    let input_vec = Sentences { line: [" ", "Hi i'm nav!"].map(String::from).to_vec()};
    */
    
    let input_vec = Sentences { line: read_file_vec("src/files/Input.txt")};

    //Print all lengths of the contents in the previous vector as a hashmap.
    if let Some(items) = &input_vec.lengths() { println!("All the items in the hashmap are: {:?}", items) }

    //Print the longest item in the Vector
    if let Some(longest) =  &input_vec.longest() {println!("[{:?}] Is the longest.", longest)}

}
