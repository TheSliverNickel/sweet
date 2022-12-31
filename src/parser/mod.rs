mod ops;
use crate::types;

pub fn parse(stack: &mut Vec<types::Element>, input: &str) {
    let typesig = input.chars().next().unwrap();

    if typesig == '"' {
        stack.push(parse_string(input));
    } else if typesig == '#' {
        if let Some(elem) = parse_basen(input) {
            stack.push(elem);
        }
    } else if typesig == '[' {
        let iter: Vec<&str> = input[1..(input.len() - 2)].split_whitespace().collect();
        let mut arr: Vec<f64> = Vec::new();
        let mut len = 0;
        for x in iter {
            if let types::Element::Number(val) = parse_int(x) {
                arr.push(val);
                len = len + 1;
            } else {
                ops::error("Invalid input [arrays are all integers]");
                return;
            }
        }
        stack.push(types::Element::Array {
            arr: arr.into_boxed_slice(),
            len,
        });
    } else if input.trim().parse::<f64>().is_ok() {
        stack.push(parse_int(input));
    } else if input.trim().eq("dep") {
        stack.push(types::Element::BaseN(stack.len(), 10));
    } else if input.trim().eq("arrto") {
        ops::arrto(stack);
    } else if input.trim().eq("toarr") {
        ops::toarr(stack);
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

fn parse_int(input: &str) -> types::Element {
    types::Element::Number(input.trim().parse::<f64>().unwrap())
}

fn parse_string(input: &str) -> types::Element {
    // We don't trim the string so string values that are made of only
    // whitespace are not discarded. Ex. " " is a valid string.
    types::Element::String(String::from(&input[1..(input.len() - 2)]))
}

fn parse_basen(input: &str) -> Option<types::Element> {
    if let Some(c) = input.trim().chars().rev().next() {
        let input = &input[1..(input.len()) - 2];
        let (val, base) = match c {
            'b' => (usize::from_str_radix(input, 2), 2),
            'o' => (usize::from_str_radix(input, 8), 8),
            'd' => (usize::from_str_radix(input, 10), 10),
            'h' => (usize::from_str_radix(input, 16), 16),
            _ => (usize::from_str_radix("5", 3), -1), // Need to generate an Err Result
        };
        if val.is_err() {
            ops::error("Invalid input");
        } else {
            return Some(types::Element::BaseN(val.unwrap(), base));
        }
    } else {
        ops::error("Invalid input")
    }
    return None;
}
