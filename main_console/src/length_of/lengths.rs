use std::collections::HashMap;

struct Sentences<'a> {
    line: Vec<&'a str>,
}

type SentenceTuple<'a> = (&'a str, usize);

trait PrintLengths{
    fn print_length(&self);
}

trait GetLengths {
    fn lengths(&self) -> Vec<SentenceTuple>;
}

impl<'a> GetLengths for Sentences<'a> {
    fn lengths(&self) -> Vec<SentenceTuple> {
        let lengths = self.line.iter().map(|x| x.len()).collect::<Vec<usize>>();
        return self
            .line
            .clone()
            .into_iter()
            .zip(lengths)
            .collect::<Vec<SentenceTuple>>();
    }
}

impl PrintLengths for Vec<SentenceTuple<'_>>{
    fn print_length(&self) {
        
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn length_tuple() {
        let input_array = Sentences{line: vec!["", " ", "Hi i'm nav."],};
        assert_eq!(input_array.lengths(), vec![("", 0), (" ", 1), ("Hi i'm nav.", 11)]);
    }
}
