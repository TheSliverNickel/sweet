use crate::types;

// It is expected that nothing happens when an error is called (no garbage on
// stack, no meaningless operations, etc.) So just warning is fine i promise.
pub fn error(error: &str) {
    println!("\nError: {error}\n");
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
            types::Element::Array { arr, len } => {
                stack.push(types::Element::Array {
                    arr: arr.clone(),
                    len,
                });
                stack.push(types::Element::Array { arr, len });
            }
        };
    }
}

pub fn swap(stack: &mut Vec<types::Element>) {
    if stack.len() < 2 {
        error("Not enough elements");
        return;
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
        return;
    }
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
        (types::Element::Number(val1), types::Element::Array { mut arr, len }) => {
            let values = arr.to_vec();
            for (index, i) in values.iter().enumerate() {
                arr[index] = i + val1;
            }
            stack.push(types::Element::Array { arr, len });
        }
        (
            types::Element::Array {
                arr: arr1,
                len: len1,
            },
            types::Element::Array {
                arr: mut arr2,
                len: len2,
            },
        ) => {
            if len1 != len2 {
		stack.push(types::Element::Array { arr: arr2, len: len2 });
		stack.push(types::Element::Array { arr: arr1, len: len1 });
                error("Mismatched lengths");
                return;
            }
            for (index, val1) in arr1.iter().enumerate() {
                arr2[index] = arr2[index] + val1;
            }
            stack.push(types::Element::Array {
                arr: arr2,
                len: len1,
            });
        }
        _ => error("Type mismatch"),
    }
}

pub fn sub(stack: &mut Vec<types::Element>) {
    if stack.len() < 2 {
        error("Not enough elements");
        return;
    }
    let var1 = stack.pop().unwrap();
    let var2 = stack.pop().unwrap();
    match (var1, var2) {
        (types::Element::Number(val1), types::Element::Number(val2)) => {
            stack.push(types::Element::Number(val2 - val1))
        }
        (types::Element::BaseN(val1, base), types::Element::BaseN(val2, _)) => {
            stack.push(types::Element::BaseN(val2 - val1, base))
        }
        (types::Element::Number(val1), types::Element::Array { mut arr, len }) => {
            let values = arr.to_vec();
            for (index, i) in values.iter().enumerate() {
                arr[index] = i - val1;
            }
            stack.push(types::Element::Array { arr, len });
        }
        (
            types::Element::Array {
                arr: arr1,
                len: len1,
            },
            types::Element::Array {
                arr: mut arr2,
                len: len2,
            },
        ) => {
            if len1 != len2 { 
		stack.push(types::Element::Array { arr: arr2, len: len2 });
		stack.push(types::Element::Array { arr: arr1, len: len1 });
                error("Mismatched lengths");
                return;
            }
            for (index, val1) in arr1.iter().enumerate() {
                arr2[index] = arr2[index] - val1;
            }
            stack.push(types::Element::Array {
                arr: arr2,
                len: len1,
            });
        }
        _ => error("Type mismatch"),
    }
}

pub fn mul(stack: &mut Vec<types::Element>) {
    if stack.len() < 2 {
        error("Not enough elements");
        return;
    }
    let var1 = stack.pop().unwrap();
    let var2 = stack.pop().unwrap();
    match (var1, var2) {
        (types::Element::Number(val1), types::Element::Number(val2)) => {
            stack.push(types::Element::Number(val2 * val1))
        }
        (types::Element::BaseN(val1, base), types::Element::BaseN(val2, _)) => {
            stack.push(types::Element::BaseN(val2 * val1, base))
        }
        (types::Element::Number(val1), types::Element::Array { mut arr, len }) => {
            let values = arr.to_vec();
            for (index, i) in values.iter().enumerate() {
                arr[index] = i * val1;
            }
            stack.push(types::Element::Array { arr, len });
        }
        (
            types::Element::Array {
                arr: arr1,
                len: len1,
            },
            types::Element::Array {
                arr: mut arr2,
                len: len2,
            },
        ) => {
            if len1 != len2 { 
		stack.push(types::Element::Array { arr: arr2, len: len2 });
		stack.push(types::Element::Array { arr: arr1, len: len1 });
                error("Mismatched lengths");
                return;
            }
            for (index, val1) in arr1.iter().enumerate() {
                arr2[index] = arr2[index] * val1;
            }
            stack.push(types::Element::Array {
                arr: arr2,
                len: len1,
            });
        }
        _ => error("Type mismatch"),
    }
}

pub fn div(stack: &mut Vec<types::Element>) {
    if stack.len() < 2 {
        error("Not enough elements");
        return;
    }
    let var1 = stack.pop().unwrap();
    let var2 = stack.pop().unwrap();
    match (var1, var2) {
        (types::Element::Number(val1), types::Element::Number(val2)) => {
            stack.push(types::Element::Number(val2 / val1))
        }
        (types::Element::BaseN(val1, base), types::Element::BaseN(val2, _)) => {
            stack.push(types::Element::BaseN(val2 / val1, base))
        }
        (types::Element::Number(val1), types::Element::Array { mut arr, len }) => {
            let values = arr.to_vec();
            for (index, i) in values.iter().enumerate() {
                arr[index] = i / val1;
            }
            stack.push(types::Element::Array { arr, len });
        }
        (
            types::Element::Array {
                arr: arr1,
                len: len1,
            },
            types::Element::Array {
                arr: mut arr2,
                len: len2,
            },
        ) => {
            if len1 != len2 { 
		stack.push(types::Element::Array { arr: arr2, len: len2 });
		stack.push(types::Element::Array { arr: arr1, len: len1 });
                error("Mismatched lengths");
                return;
            }
            for (index, val1) in arr1.iter().enumerate() {
                arr2[index] = arr2[index] / val1;
            }
            stack.push(types::Element::Array {
                arr: arr2,
                len: len1,
            });
        }
        _ => error("Type mismatch"),
    }
}

pub fn toarr(stack: &mut Vec<types::Element>) {
    if let None = stack.last() {
        error("Not enough elements");
        return;
    }

    let dep = match stack.pop().unwrap() {
        types::Element::BaseN(i, _) => i,
        a => {
            stack.push(a);
            error("Type mismatch");
            return;
        }
    };

    if stack.len() < dep as usize {
        error("Not enough elements");
        return;
    }

    let mut stack_slice = stack.split_off(stack.len() - (dep as usize));
    let mut arr: Vec<f64> = Vec::new();
    let mut ok: bool = true;

    for elem in stack_slice.iter() {
        match elem {
            types::Element::Number(i) => arr.push(*i),
            _ => {
                ok = false;
                error("Type mismatch");
            }
        }
    }

    if ok {
        stack.push(types::Element::Array {
            arr: arr.into_boxed_slice(),
            len: dep as u8,
        });
        return;
    }

    stack.append(&mut stack_slice);
}

pub fn arrto(stack: &mut Vec<types::Element>) {
    if let None = stack.last() {
	error("Not enough elements");
	return;
    }

    if let types::Element::Array { arr, len: _ } = stack.pop().unwrap() {
	for elem in arr.iter() {
	    stack.push(types::Element::Number(*elem));
	}
    }
}
