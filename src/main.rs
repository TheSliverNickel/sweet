use std::fmt::Display;
use std::io;

enum Element {
    Number(f64),
    String(String),
    BaseN(i64, i8),
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::String(i) => write!(f, "\"{i}\""),
            Element::Number(i) => write!(f, "{i}"),

            Element::BaseN(i, 2) => write!(f, "#{:b}b", i),
            Element::BaseN(i, 8) => write!(f, "#{:o}o", i),
            Element::BaseN(i, 10) => write!(f, "#{}d", i),
            Element::BaseN(i, 16) => write!(f, "#{:x}h", i),
            Element::BaseN(_, _) => write!(f, "Something has gone wrong!"),
	    // Ideally other bases would be caught in parsing.
        }
    }
}

fn disp(stack: &Vec<Element>) {
    for (index, element) in stack.iter().rev().enumerate().rev() {
        println!("{} :\t{}", index + 1, element);
    }
    print!("\n");
}

// It is expected that nothing happens when an error is called (no garbage on
// stack, no meaningless operations, etc.) So just warning is fine i promise.
fn error(error: &str) {
    println!("\n{error}\n");
}

fn parse(stack: &mut Vec<Element>, input: &String) {
    let typesig = input.chars().next().unwrap();

    if typesig == '"' {
        let elem = Element::String(String::from(&input[1..(input.len() - 2)]));
        stack.push(elem);
    } else if typesig == '#' {
        if let Some(c) = input.chars().rev().next() {
            let input = &input[1..(input.len()) - 1];
            let (val, base) = match c {
                'b' => (i64::from_str_radix(input, 2), 2),
                'o' => (i64::from_str_radix(input, 8), 8),
                'd' => (i64::from_str_radix(input, 10), 10),
                'h' => (i64::from_str_radix(input, 16), 16),
                _ => (i64::from_str_radix("h", 1), -1),
            };

            if val.is_err() {
                error("Invalid input")
            } else {
                let elem = Element::BaseN(val.unwrap(), base);
                stack.push(elem);
            }
        } else {
            error("Invalid input")
        }
    } else if input.trim().parse::<f64>().is_ok() {
        let elem = Element::Number(input.trim().parse::<f64>().unwrap());
        stack.push(elem);
    } else if input.trim().eq("+") {
        add(stack);
    } else if input.trim().eq("-") {
        sub(stack);
    } else if input.trim().eq("*") {
        mul(stack);
    } else if input.trim().eq("/") {
        div(stack);
    } else if input.eq("\n") {
        dup(stack);
    } else {
        error("Invalid input")
    }
}

fn dup(stack: &mut Vec<Element>) {
    if let Some(val) = stack.pop() {
        match val {
            Element::Number(a) => {
                stack.push(Element::Number(a));
                stack.push(Element::Number(a));
            }
            Element::String(a) => {
                stack.push(Element::String(a.clone()));
                stack.push(Element::String(a));
            }
            Element::BaseN(a, b) => {
                stack.push(Element::BaseN(a, b));
                stack.push(Element::BaseN(a, b));
            }
        };
    }
}

fn add(stack: &mut Vec<Element>) {
    if stack.len() < 2 {
        error("Not enough elements");
    } else {
        let var1 = stack.pop().unwrap();
        let var2 = stack.pop().unwrap();
        match (var1, var2) {
            (Element::Number(val1), Element::Number(val2)) => {
                stack.push(Element::Number(val2 + val1))
            }
            (Element::String(val1), Element::String(val2)) => {
                stack.push(Element::String(val2 + &val1))
            }
            (Element::BaseN(val1, _), Element::BaseN(val2, base)) => {
                stack.push(Element::BaseN(val2 + val1, base))
            }
            _ => error("Type mismatch"),
        }
    }
}

fn sub(stack: &mut Vec<Element>) {
    if stack.len() < 2 {
        error("Not enough elements");
    } else {
        let var1 = stack.pop().unwrap();
        let var2 = stack.pop().unwrap();
        match (var1, var2) {
            (Element::Number(val1), Element::Number(val2)) => {
                stack.push(Element::Number(val2 - val1))
            }
            (Element::BaseN(val1, _), Element::BaseN(val2, base)) => {
                stack.push(Element::BaseN(val2 - val1, base))
            }
            _ => error("Type mismatch"),
        }
    }
}

fn mul(stack: &mut Vec<Element>) {
    if stack.len() < 2 {
        error("Not enough elements");
    } else {
        let var1 = stack.pop().unwrap();
        let var2 = stack.pop().unwrap();
        match (var1, var2) {
            (Element::Number(val1), Element::Number(val2)) => {
                stack.push(Element::Number(val2 * val1))
            }
            (Element::BaseN(val1, _), Element::BaseN(val2, base)) => {
                stack.push(Element::BaseN(val2 * val1, base))
            }
            _ => error("Type mismatch"),
        }
    }
}

fn div(stack: &mut Vec<Element>) {
    if stack.len() < 2 {
        error("Not enough elements");
    } else {
        let var1 = stack.pop().unwrap();
        let var2 = stack.pop().unwrap();
        match (var1, var2) {
            (Element::Number(val1), Element::Number(val2)) => {
                stack.push(Element::Number(val2 / val1))
            }
            (Element::BaseN(val1, _), Element::BaseN(val2, base)) => {
                stack.push(Element::BaseN(val2 / val1, base))
            }
            _ => error("Type mismatch"),
        }
    }
}

fn main() {
    let mut stack: Vec<Element> = Vec::new();
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

        parse(&mut stack, &input);
        disp(&stack);
    }

    println!("\n\nThank you!!");
}
