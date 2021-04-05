use std::io::{self, Write};

mod parsemath;
use parsemath::{
    ast,
    parser::{ParseError, Parser},
};

fn evaluate(expr: &str) -> Result<f64, ParseError> {
    let mut parser = Parser::new(&expr)?;
    let ast = parser.parse()?;
    Ok(ast::eval(ast)?)
}

fn main() -> Result<(), ParseError> {
    println!("Please enter an arithmetic expression:");
    let mut buffer = String::new();
    loop {
        print!(">>> ");
        let _ = io::stdout().flush();
        buffer.clear();
        if let Ok(_) = io::stdin().read_line(&mut buffer) {
            let expr = buffer.split_whitespace().collect::<String>();
            let val = match evaluate(&expr) {
                Ok(val) => val,
                Err(error) => {
                    eprintln!("There was an error: {:?}", error);
                    continue;
                }
            };
            println!("{}", val);
        } else {
            break;
        }
    }
    Ok(())
}
