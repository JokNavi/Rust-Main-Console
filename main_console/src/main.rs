use std::io::Write;

fn return_two() -> i8 {
    2
}

fn main() {
    let mut line = String::new();
    print!("Enter your name: ");
    let _ = std::io::stdout().flush();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line);
    println!("no of bytes read , {}", b1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_hi() {
        println!("Hi!");
    }

    #[test]
    fn equals_2() {
        assert_eq!(return_two(), 2);
    }
}
