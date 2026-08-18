use std::io::{Write, stdin, stdout};

use bad_calculator::evaluat_expression;

fn main() {
    let mut buffer = String::new();
    let stdin = stdin();

    loop {
        print!("> ");
        stdout().flush().unwrap();

        buffer.clear();
        if stdin.read_line(&mut buffer).is_err() {
            break;
        }

        let input = buffer.trim();
        if input.is_empty() {
            continue;
        }

        if input == "exit" {
            break;
        }

        match evaluat_expression(input) {
            Ok(result) => println!("{result}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}
// Traits  look remarkably like Java Interface/abstract classes,
// They define method signtures that type must implement.
// Drop trait provide cleanup functionality that seems equivalent to destructors.
//
//
// Structs are not classes are data Rust Data Structure
//
// You can't extend or modify their definition by creating children of the struct.
// A Rust struct is what it is.
//
// A traits are much more accuarately thought of as contracts.
//
// Polymorphism is build into the essence of classes in most modern OO Language. There is an
// inherent expectation that child classes are interchangeable with their parents and siblings.
//
// The Borrow checker prevents object patterns
