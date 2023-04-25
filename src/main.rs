use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

mod eval;
use crate::eval::eval;

fn main()  {
    // `()` can be used when no completer is required
    let res = repl();
    match res{
        Ok(()) => {
            println!("exited.");
        },
        Err(e) => {
            println!("terminated! {}",e);
        },
    }
}

fn repl() -> Result<()>{
    let mut rl = DefaultEditor::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _read = rl.add_history_entry(line.as_str());
                match _read{
                    Ok(true) => {
                        if line.len() > 0 {
                            let _result = eval(line.as_str());
                            println!("{}",_result);
                        }
                    },
                    _ => {
                        break;
                    },
                }
                
            },
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    let _save = rl.save_history("history.txt");
    match _save{
        Err(e) => {
            return Err(e);
        },
        _ => {
            return Ok(());
        },
    }
}