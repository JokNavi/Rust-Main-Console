
mod length_of;
//use length_of::lengths::print_str_length;
use length_of::lengths::print_length;

fn main() {
    print_length();
}


#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn equals_2() {
        assert_eq!(2, 2);
    }
}
