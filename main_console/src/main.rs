mod length_of;
use length_of::lengths::print_lengths;

pub struct Sentences<'a> {
    line: Vec<&'a str>,
}


fn main() {
    print_lengths(&Sentences{line: vec!["", " ", "Hi i'm nav."]});
}