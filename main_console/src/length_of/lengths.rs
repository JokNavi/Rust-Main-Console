use crate::Sentences;
use std::collections::HashMap;

type SentenceLengths = HashMap<String, usize>;
pub trait GetLengths {
    fn lengths(&self) -> Option<SentenceLengths>;
}

pub trait GetLongestItem {
    fn longest(&self) -> Option<String>;
}

impl GetLengths for Sentences {
    fn lengths(&self) -> Option<SentenceLengths> {
        match self.line.len() {
            0 => None,
            _ => {
                //Is used to get each induvidual length so it can be zipped together and returned
                let lengths = self.line.iter().map(|x| x.len()).collect::<Vec<usize>>();
                //Merges original strings and length together into hashmap.
                Some(
                    self.line.clone()
                        .into_iter()
                        .map(String::from)
                        .zip(lengths)
                        .collect::<SentenceLengths>(),
                )
            }
        }
    }
}

impl GetLongestItem for Sentences {
    fn longest(&self) -> Option<String> {
        match self.line.len() {
            0 => None,
            _ => {
                //finds the longest item in vectors of type Sentences
                let longest = self.line.iter().fold(&self.line[0], |acc, item| {
                    if item.len() > acc.len() {
                        item
                    } else {
                        acc
                    }
                });
                return Some(longest.to_string());
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let string_lengths = Sentences { line: vec![] };
        if string_lengths.lengths() != None {
            panic!("Empty Vec was not handled well.")
        }
    }

    #[test]
    fn empty_value() {
        let string_lengths = Sentences { line: vec![""] };
        if let Some(result) = string_lengths.lengths() {
            assert_eq!(result[""], 0)
        };
    }

    #[test]
    fn whitespace() {
        let string_lengths = Sentences { line: vec![" "] };
        if let Some(result) = string_lengths.lengths() {
            assert_eq!(result[" "], 1)
        };
    }

    #[test]
    fn special_characters() {
        let string_lengths = Sentences {
            line: vec!["&'\n\\."],
        };
        if let Some(result) = string_lengths.lengths() {
            assert_eq!(result["&'\n\\."], 5)
        };
    }
}
