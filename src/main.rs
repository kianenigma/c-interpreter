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

mod constants;
use crate::constants::*;

mod config;
use crate::config::Config;

mod common;

fn main() {
  let mut program = Program {
    defines: vec![], 
    includes: vec![], 
    statements: vec![], 
    functions: vec![],
    last_push: StmsType::Stmt,
    argv: String::from("")
  };

  let mut conf = Config { 
    cc: "gcc".to_string() 
  };

  program.populate_default();
  
  let mut rl = Editor::<()>::new();
  println!("{}{}", ASCII, HELP);

  loop {
    let readline = rl.readline(PROMPT);
    let mut current_statement = match readline {
      Ok(line) => {
          rl.add_history_entry(line.as_ref());
          line
      },
      Err(ReadlineError::Interrupted) => {
          println!("CTRL-C Signal. Cleaning. use CTRL-D to exit.");
          continue;
      },
      Err(ReadlineError::Eof) => {
          break;
      },
      Err(err) => {
          println!("Error: {:?}", err);
          break
      }
    };

    current_statement = current_statement.trim().to_string();

    if &current_statement[0..1] == "~" {
      match execute_command(&current_statement, &mut program, &mut conf) {
        Ok(msg) => println!("Command successfull => \n{}", msg),
        Err(why) => println!("{} => {}", "Command failed".bold().red(), why)
      }
      continue;
    }
    

    let stmt_type: StmsType = get_statement_type(&current_statement);    
    program.push(&current_statement, stmt_type); 

    let begin = Instant::now();
    match program.run(&conf) {
      Err(why) => { 
        println!("{}: {}", "--- Error Type:".red().bold(), why);
        program.pop();
      },
      Ok(handle) => {
        println!("{}", format_output_handle(&handle, begin));
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