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

fn main() {
  let mut program = Program {
    defines: vec![], 
    includes: vec![], 
    statements: vec![], 
    last_push: StmsType::Stmt,
    argv: String::from("")
  };
  program.populate_default();
  
  let mut rl = Editor::<()>::new();

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
      _ => StmsType::Stmt
  }
}