use std::fmt::Display;

pub enum Element {
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

pub fn disp(stack: &Vec<Element>) {
    for (index, element) in stack.iter().rev().enumerate().rev() {
        println!("{} :\t{}", index + 1, element);
    }
    print!("\n");
}
