extern crate codespan;
extern crate codespan_reporting;
extern crate rustyline;
extern crate sparc;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use sparc::{Env, ExprParser};

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let parser = ExprParser::new();
    let env = Env::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());

                let expr = parser.parse(&line);
                let expr = match expr {
                    Ok(expr) => {
                        println!("Expr: {:?}", expr);
                        expr
                    }
                    Err(e) => {
                        println!("Parse error: {}", e);
                        break;
                    }
                };

                let result = env.eval_expr(&expr);
                match result {
                    Ok(result) => {
                        println!("Result: {:?}", result);
                    }
                    Err(e) => {
                        println!("Runtime error: {:?}", e);
                    }
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
