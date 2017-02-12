extern crate rustyline;

use std::process::Command;
use std::env;
use std::path::Path;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {
        println!("No previous history.");
    }
    loop {
        // Prompt
        let readline = rl.readline("wish #(");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                // Eval
                let mut cmd_tokens = line.split_whitespace();
                if let Some(cmd) = cmd_tokens.next() {
                    if cmd == "cd" {
                        cd(cmd_tokens.next());
                    } else {
                        let args: Vec<_> = cmd_tokens.collect();
                        let status = Command::new(cmd)
                            .args(&args)
                            .status()
                            .expect("Failed to execute child");
                            if !status.success() {
                                if let Some(ec) = status.code() {
                                    println!("{}", ec);
                                }
                            }
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("Kill");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("Exit");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

fn cd(path: Option<&str>) {
    if let Some(p) = path {
        if env::set_current_dir(Path::new(p)).is_err() {
            println!("No such file or directory: {}", p);
        }
    } else {
        let home = env::home_dir();
        if let Some(h) = home {
            if env::set_current_dir(h.as_path()).is_err() {
                println!("Can't access home directory");
            }
        }
    }
}
