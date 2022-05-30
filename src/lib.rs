#![feature(proc_macro_quote)]
use proc_macro::TokenStream;

/// Macro for creating functions from equations
#[proc_macro]
pub fn equation(input: TokenStream) -> TokenStream {
    let mut equation = input.to_string();
    let mut equation_offset = 0;
    let mut last_char = equation.clone().chars().nth(0).unwrap();
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
        // if a variable is not in the variables variable then add it
        if c.is_alphabetic() && !variables.0.contains(&c) {
            variables.0.push(c);
            variables.1 += 1;
        }
        last_char = c;
    }

    // proc macro stuff
    let equation_clone = equation.clone();
    let equation_split = equation_clone.split(";").collect::<Vec<&str>>();
    let var_string_prec = format!("{:?} ", variables.0)
        .replace("[", "")
        .replace("]", "")
        .replace(",", "")
        .replace("'", "");
    let var_string;
    let equation_split_len = equation_split.len().clone();
    // if there is a "; <type>" add the type to the variables
    if equation_split_len != 1 {
        var_string = var_string_prec.replace(" ", format!(": {},", equation_split[1]).as_str())
    } else {
        var_string = var_string_prec.replace(" ", ": i32,");
    }
    return format!("|{var_string}| {{ return ({equation}); }}")
        .parse()
        .unwrap();
}
