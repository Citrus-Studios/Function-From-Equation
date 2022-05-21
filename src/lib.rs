#![feature(proc_macro_quote)]
use proc_macro::{quote, TokenStream};

/// Macro for creating functions from equations
#[proc_macro]
pub fn equation(input: TokenStream) -> TokenStream {
    let mut equation = input.to_string();
    let mut equation_offset = 0;
    let mut last_char = equation.chars().nth(0).unwrap();
    let mut variables = (vec![], 0);
    for (i, c) in equation.clone().chars().into_iter().enumerate() {
        // if a variable is next to a number add a * inbetween the number and the variable
        if c.is_alphabetic()
            && last_char.is_numeric()
            && (c != '+' || c != '*' || c != '/' || c != '-')
        {
            equation.insert(i + equation_offset, '*');
            equation_offset += 1;
        }
        if c.is_alphabetic() && !variables.0.contains(&c) {
            variables.0.push(c);
            variables.1 += 1;
        }
        last_char = c;
    }
    println!("Variables Amount: {variables}", variables = variables.1);
    let var_string: TokenStream = format!("{:?} ", variables.0)
        .replace("[", "")
        .replace("]", "")
        .replace(",", "")
        .replace("'", "")
        .replace(" ", ": i32, ")
        .parse()
        .unwrap();
    println!("{}", format!("|{var_string}| {{ return ({equation}); }}"));
    return format!("|{var_string}| {{ return ({equation}); }}")
        .parse()
        .unwrap();
}
