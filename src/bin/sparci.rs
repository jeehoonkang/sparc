extern crate codespan;
extern crate codespan_reporting;
extern crate rustyline;
extern crate sparc;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use sparc::Executor;

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let executor = Executor::new();

    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());

                match executor.exec(&line) {
                    Ok(value) => {
                        println!("{:?}", value);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
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
                println!("IO error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
