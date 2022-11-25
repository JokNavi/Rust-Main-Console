use crate::Sentences;
use std::collections::HashMap;

type SentenceLengths<'a> = HashMap<&'a str, usize>;
pub trait GetLengths {
    fn lengths(&self) -> Result<SentenceLengths, &'static str>;
}

pub trait GetLongestItem {
    fn longest(&self) -> Result<&str, &'static str>;
}

impl<'a> GetLengths for Sentences<'a> {
    fn lengths(&self) -> Result<SentenceLengths, &'static str> {
        match self.line.len() {
            0 => Err("Vector is empty"),
            _ => {
                let lengths = self.line.iter().map(|x| x.len()).collect::<Vec<usize>>();
                //Is used to get each induvidual length so it can be zipped together and returned
                Ok(self
                    .line
                    .clone()
                    .into_iter()
                    .zip(lengths)
                    .collect::<SentenceLengths>())
            }
        }
    }
}

impl<'a> GetLongestItem for Sentences<'a> {
    fn longest(&self) -> Result<&str, &'static str> {
        match self.line.len() {
            0 => Err("Vector is empty"),
            _ => {
                //finds the longest item in vectors of type Sentences
                let longest = self.line.iter().fold(self.line[0], |acc, &item| {
                    if item.len() > acc.len() {
                        item
                    } else {
                        acc
                    }
                });
                Ok(longest)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_longest() {
        let string_lengths = Sentences { line: vec![] };
        assert_eq!(string_lengths.longest(), Err("Vector is empty"));
    }

    #[test]
    fn empty_length() {
        let string_lengths = Sentences { line: vec![] };
        assert_eq!(string_lengths.lengths(), Err("Vector is empty"));
    }

    #[test]
    fn empty_value() {
        let string_lengths = Sentences { line: vec![""] };
        assert_eq!(string_lengths.lengths().unwrap()[""], 0);
    }

    #[test]
    fn whitespace() {
        let string_lengths = Sentences { line: vec![" "] };
        assert_eq!(string_lengths.lengths().unwrap()[" "], 1);
    }

    #[test]
    fn special_characters() {
        let string_lengths = Sentences {
            line: vec!["&'\n\\."],
        };
        assert_eq!(string_lengths.lengths().unwrap()["&'\n\\."], 5);
    }
}
