use std::io;

mod parser;
pub mod types;

fn main() {
    let mut stack: Vec<types::Element> = Vec::new();
    loop {
        // Get and trim input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input");

        // Check to make sure that the string isn't empty
        if input.chars().next().is_none() {
            break;
        }

        parser::parse(&mut stack, &input);
        types::disp(&stack);
    }

    println!("\n\nThank you!!");
}
