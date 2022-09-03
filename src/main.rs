use std::env;

fn main() {
    let args = cli_args();
    fizzbuzz_selector(args);
}

fn cli_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    // Check if arg is given and if not print help
    if args.len() < 2 {
        println!("Err: 1 arg is required");
        println!("Usage: cargo run <number>\n   or: main <number>");
        println!("Supported arguments\n\t1: fizzbuzz\n\t2: fizzbuzz with match\n\t3: <tbd>");
        std::process::exit(1);
    }
    return args;
}

fn fizzbuzz_selector(args: Vec<String>) {
    match args[1].as_str() {
        "1" => fizzbuzz(),
        "2" => fizzbuzz_match(),
        "3" => fizzbuzz_idiomatic(),
        _ => println!("Err: Invalid arg"),
    }
}

fn fizzbuzz() {
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }

}

fn fizzbuzz_match() {
    for i in 1..101 {
        match(i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i),
        }
    }
}

// Control func for callin fizzy()
fn fizzbuzz_idiomatic() {
    for i in 1..101 {
        println!("{}", i.fizzy());
    }
}

// Trait for fizzy()
pub trait Fizzy {
    fn fizzy(&self) -> String;
}

// Implement fizzy()
impl Fizzy for i32 {
    fn fizzy(&self) -> String {
        match(self % 3, self % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            (_, _) => self.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fizz() {
        for i in &[3, 6, 27] {
            assert_eq!(i.fizzy(), "Fizz");
        }
    }
    #[test]
    fn test_buzz() {
        for i in &[5, 10, 25] {
            assert_eq!(i.fizzy(), "Buzz");
        }
    }
    #[test]
    fn test_fizzbuzz() {
        for i in &[15, 30, 45] {
            assert_eq!(i.fizzy(), "FizzBuzz");
        }
    }
    #[test]
    fn test_numbers(){
        for i in &[1, 2, 4, 13, 29, 98] {
            assert_eq!(i.fizzy(), i.to_string());
        }
    }
}