pub fn print_str_length(input: String){
    //Removes the trailing newline for accurate length calculations.
    println!("{} Is {} characters long.", input.trim(), input.trim().len())
}
