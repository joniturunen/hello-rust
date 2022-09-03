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
        "3" => println!("TBA"),
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