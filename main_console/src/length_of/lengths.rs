struct Sentences<'a>{line: Vec<&'a str>}

trait GetLengths {
    fn lengths(&self) -> Vec<(&str, usize)>;
}

impl<'a> GetLengths for Sentences<'a> {
    fn lengths(&self) -> Vec<(&str, usize)> {
        let lengths = self.line.iter().map(|x| x.len()).collect::<Vec<usize>>();
        self.line.clone().into_iter().zip(lengths).collect::<Vec<(&str, usize)>>()
    }
}


pub fn print_length() {
    let  input_array = Sentences{line: vec!["Hi", "hello", "Hi i'm nav"]};
    println!("{:?}", input_array.lengths());
}
