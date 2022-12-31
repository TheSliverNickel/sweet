mod ops;
use crate::types;

pub fn parse(stack: &mut Vec<types::Element>, input: &str) {
    let typesig = input.chars().next().unwrap();

    if typesig == '"' {
	// We don't trim the string so string values that are made of only
	// whitespace are not discarded. Ex. " " is a valid string.
        let elem = types::Element::String(String::from(&input[1..(input.len() - 2)]));
        stack.push(elem);
    } else if typesig == '#' {
        if let Some(c) = input.trim().chars().rev().next() {
            let input = &input[1..(input.len()) - 2];
            let (val, base) = match c {
                'b' => (i64::from_str_radix(input, 2), 2),
                'o' => (i64::from_str_radix(input, 8), 8),
                'd' => (i64::from_str_radix(input, 10), 10),
                'h' => (i64::from_str_radix(input, 16), 16),
                _ => (i64::from_str_radix("5", 3), -1), // Need to generate an Err Result
            };
            if val.is_err() {
                ops::error("Invalid input")
            } else {
                let elem = types::Element::BaseN(val.unwrap(), base);
                stack.push(elem);
            }
        } else {
            ops::error("Invalid input")
        }
    } else if input.trim().parse::<f64>().is_ok() {
        let elem = types::Element::Number(input.trim().parse::<f64>().unwrap());
        stack.push(elem);
    } else if input.trim().eq("+") {
        ops::add(stack);
    } else if input.trim().eq("-") {
        ops::sub(stack);
    } else if input.trim().eq("*") {
        ops::mul(stack);
    } else if input.trim().eq("/") {
        ops::div(stack);
    } else if input.eq("\t\n") {
	ops::swap(stack);
    } else if input.eq("\n") {
        ops::dup(stack);
    } else {
        ops::error("Invalid input")
    }
}
