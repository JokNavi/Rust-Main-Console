use std::collections::HashMap;
use crate::Sentences;

type SentenceLengths<'a> = HashMap<&'a str, usize>;
pub trait GetLengths {
    fn lengths(&self) -> SentenceLengths;
}

impl<'a> GetLengths for Sentences<'a> {
    fn lengths(&self) -> SentenceLengths {
        let lengths = self.line.iter().map(|x| x.len()).collect::<Vec<usize>>();
        return self
            .line
            .clone()
            .into_iter()
            .zip(lengths)
            .collect::<SentenceLengths>();
    }
}

pub fn print_lengths(input: &Sentences) {
    for (sentence, length) in input.lengths(){
        println!("\"{}\", is: {} long.", sentence, length)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let string_lengths = Sentences {line: vec![""]};
        assert_eq!(string_lengths.lengths()[""],0);
    }

    #[test]
    fn whitespace() {
        let string_lengths = Sentences {line: vec![" "]};
        assert_eq!(string_lengths.lengths()[" "],1);
    }

    #[test]
    fn special_characters() {
        let string_lengths = Sentences {line: vec!["&'\n\\."]};
        assert_eq!(string_lengths.lengths()["&'\n\\."],5);
    }
}

