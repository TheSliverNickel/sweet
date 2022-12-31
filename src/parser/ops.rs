use crate::types;

// It is expected that nothing happens when an error is called (no garbage on
// stack, no meaningless operations, etc.) So just warning is fine i promise.
pub fn error(error: &str) {
    println!("\n{error}\n");
}

pub fn dup(stack: &mut Vec<types::Element>) {
    if let Some(val) = stack.pop() {
        match val {
            types::Element::Number(a) => {
                stack.push(types::Element::Number(a));
                stack.push(types::Element::Number(a));
            }
            types::Element::String(a) => {
                stack.push(types::Element::String(a.clone()));
                stack.push(types::Element::String(a));
            }
            types::Element::BaseN(a, b) => {
                stack.push(types::Element::BaseN(a, b));
                stack.push(types::Element::BaseN(a, b));
            }
        };
    }
}

pub fn swap(stack: &mut Vec<types::Element>) {
    if stack.len() < 2 {
	error("Not enough elements");
    } else {
	// No chance of a None. Unwrap away.
	let var1 = stack.pop().unwrap();
	let var2 = stack.pop().unwrap();
	stack.push(var1);
	stack.push(var2);
    }
}

pub fn add(stack: &mut Vec<types::Element>) {
    if stack.len() < 2 {
        error("Not enough elements");
    } else {
        let var1 = stack.pop().unwrap();
        let var2 = stack.pop().unwrap();
        match (var1, var2) {
            (types::Element::Number(val1), types::Element::Number(val2)) => {
                stack.push(types::Element::Number(val2 + val1))
            }
            (types::Element::String(val1), types::Element::String(val2)) => {
                stack.push(types::Element::String(val2 + &val1))
            }
            (types::Element::BaseN(val1, base), types::Element::BaseN(val2, _)) => {
                stack.push(types::Element::BaseN(val2 + val1, base))
            }
            _ => error("Type mismatch"),
        }
    }
}

pub fn sub(stack: &mut Vec<types::Element>) {
    if stack.len() < 2 {
        error("Not enough elements");
    } else {
        let var1 = stack.pop().unwrap();
        let var2 = stack.pop().unwrap();
        match (var1, var2) {
            (types::Element::Number(val1), types::Element::Number(val2)) => {
                stack.push(types::Element::Number(val2 - val1))
            }
            (types::Element::BaseN(val1, base), types::Element::BaseN(val2, _)) => {
                stack.push(types::Element::BaseN(val2 - val1, base))
            }
            _ => error("Type mismatch"),
        }
    }
}

pub fn mul(stack: &mut Vec<types::Element>) {
    if stack.len() < 2 {
        error("Not enough elements");
    } else {
        let var1 = stack.pop().unwrap();
        let var2 = stack.pop().unwrap();
        match (var1, var2) {
            (types::Element::Number(val1), types::Element::Number(val2)) => {
                stack.push(types::Element::Number(val2 * val1))
            }
            (types::Element::BaseN(val1, base), types::Element::BaseN(val2, _)) => {
                stack.push(types::Element::BaseN(val2 * val1, base))
            }
            _ => error("Type mismatch"),
        }
    }
}

pub fn div(stack: &mut Vec<types::Element>) {
    if stack.len() < 2 {
        error("Not enough elements");
    } else {
        let var1 = stack.pop().unwrap();
        let var2 = stack.pop().unwrap();
        match (var1, var2) {
            (types::Element::Number(val1), types::Element::Number(val2)) => {
                stack.push(types::Element::Number(val2 / val1))
            }
            (types::Element::BaseN(val1, base), types::Element::BaseN(val2, _)) => {
                stack.push(types::Element::BaseN(val2 / val1, base))
            }
            _ => error("Type mismatch"),
        }
    }
}
