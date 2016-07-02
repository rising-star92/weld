extern crate weld;

use std::io::{stdin, stdout, Write};
use weld::grammar::*;
use weld::pretty_print::*;
use weld::type_inference::*;
use weld::macro_processor;

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        if &line == "" {
            println!("");
            return  // Reached EOF
        }

        let trimmed = line.trim();
        if trimmed == "" {
            continue;
        }

        let program = parse_program(trimmed);
        if let Err(ref e) = program {
            println!("Error during parsing: {:?}", e);
            continue;
        }
        let program = program.unwrap();
        println!("Raw structure:\n{:?}\n", program);

        let expr = macro_processor::process_program(&program);
        if let Err(ref e) = expr {
            println!("Error during macro substitution: {}", e);
            continue;
        }
        let mut expr = expr.unwrap();
        println!("After macro substitution:\n{}\n", print_expr(&expr));

        if let Some(ref e) = infer_types(&mut expr).err() {
            println!("Error during type inference: {}", e);
            continue;
        }
        println!("After type inference:\n{}\n", print_typed_expr(&expr));
        println!("Expression type: {}\n", print_type(&expr.ty));
    }
}