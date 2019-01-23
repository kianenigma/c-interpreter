extern crate colored; 
extern crate rustyline;

// std imports 
use std::time::{Instant};

// external crate imports 
use colored::*;

use rustyline::error::ReadlineError;
use rustyline::Editor;

// local crate import 
mod program;
use crate::program::{Program, StmsType};

mod command;
use crate::command::*;

static PROMPT: &'static str = ">>> ";
static ASCII: &'static str = r#"
  /$$$$$$                      /$$$$$$ /$$   /$$ /$$$$$$$$/$$$$$$$$ /$$$$$$$  /$$$$$$$  /$$$$$$$  /$$$$$$$$/$$$$$$$$/$$$$$$$$ /$$$$$$$ 
 /$$__  $$                    |_  $$_/| $$$ | $$|__  $$__/ $$_____/| $$__  $$| $$__  $$| $$__  $$| $$_____/__  $$__/ $$_____/| $$__  $$
| $$  \__/                      | $$  | $$$$| $$   | $$  | $$      | $$  \ $$| $$  \ $$| $$  \ $$| $$        | $$  | $$      | $$  \ $$
| $$             /$$$$$$        | $$  | $$ $$ $$   | $$  | $$$$$   | $$$$$$$/| $$$$$$$/| $$$$$$$/| $$$$$     | $$  | $$$$$   | $$$$$$$/
| $$            |______/        | $$  | $$  $$$$   | $$  | $$__/   | $$__  $$| $$____/ | $$__  $$| $$__/     | $$  | $$__/   | $$__  $$
| $$    $$                      | $$  | $$\  $$$   | $$  | $$      | $$  \ $$| $$      | $$  \ $$| $$        | $$  | $$      | $$  \ $$
|  $$$$$$/                     /$$$$$$| $$ \  $$   | $$  | $$$$$$$$| $$  | $$| $$      | $$  | $$| $$$$$$$$  | $$  | $$$$$$$$| $$  | $$
 \______/                     |______/|__/  \__/   |__/  |________/|__/  |__/|__/      |__/  |__/|________/  |__/  |________/|__/  |__/
"#;

const HELP: &'static str = r#"
Supported statements:
    - Includes
    - Defines 
    - Functions (must be prepended with `#fun`)
    - Normal Statement.
Commands: 
    ~src
    ~run
    ~del <X>
    ~arg <X>

by Kianenigma.
"#;
fn main() {
  let mut program = Program {
    defines: vec![], 
    includes: vec![], 
    statements: vec![], 
    functions: vec![],
    last_push: StmsType::Stmt,
    argv: String::from("")
  };
  program.populate_default();
  
  let mut rl = Editor::<()>::new();
  println!("{}{}", ASCII, HELP);

  loop {
    let readline = rl.readline(PROMPT);
    let current_statement = match readline {
      Ok(line) => {
          rl.add_history_entry(line.as_ref());
          line
      },
      Err(ReadlineError::Interrupted) => {
          println!("CTRL-C");
          break
      },
      Err(ReadlineError::Eof) => {
          println!("CTRL-D");
          break
      },
      Err(err) => {
          println!("Error: {:?}", err);
          break
      }
    };

    match execute_command(&current_statement, &mut program) {
      Ok(_) => { continue; },
      Err(_) => ()
    };

    let stmt_type: StmsType = get_statement_type(&current_statement);    
    program.push(&current_statement, stmt_type); 

    let begin = Instant::now();
    match program.run() {
      Err(why) => { 
        println!("{}: {}", "--- Error".red(), why);
        program.pop();
      },
      Ok(handle) => {
        print_output_handle(&handle, begin);
      }
    }
  }
}


fn get_statement_type(stms: &str) -> StmsType {
  match &stms[0..4] {
      "#inc" => StmsType::Inc,
      "#def" => StmsType::Def,
      "#fun" => StmsType::Func,
      _ => StmsType::Stmt
  }
}