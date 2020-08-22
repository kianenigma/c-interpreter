use colored::*;
use rustyline::{error::ReadlineError, Editor};
use std::time::Instant;

mod command;
mod config;
mod constants;
mod program;
mod testing_utils;

use crate::command::*;
use crate::config::Config;
use crate::constants::*;
use crate::program::{Program, StatementType};

fn main() {
	let mut program = Program::default();
	let mut conf = Config::default();

	let mut rl = Editor::<()>::new();
	println!("{}{}", ASCII, HELP);

	loop {
		let line_result = rl.readline(PROMPT);
		let current_statement = match line_result {
			Ok(line) => {
				rl.add_history_entry(&line);
				line
			}
			Err(ReadlineError::Interrupted) => {
				println!("CTRL-C Signal. Cleaning. use CTRL-D to exit.");
				continue;
			}
			Err(ReadlineError::Eof) => {
				break;
			}
			Err(err) => {
				println!("Error: {:?}", err);
				break;
			}
		};

		interpret(current_statement, &mut program, &mut conf);
	}
}

fn interpret(input: String, program: &mut Program, conf: &mut Config) {
	let current_statement = input.trim().to_string();

	if current_statement.starts_with("~") {
		match execute_command(&current_statement, program, conf) {
			Ok(msg) => println!("Command successfull => \n{}", msg),
			Err(why) => println!("{} => {}", "Command failed".bold().red(), why),
		}
		return ();
	}

	let stmt_type = StatementType::from(current_statement.as_str());
	program.push(&current_statement, stmt_type);

	let begin = Instant::now();
	match program.run(&conf) {
		Err(why) => {
			println!("{}: {}", "--- Error Type:".red().bold(), why);
			program.pop();
		}
		Ok(handle) => {
			println!("{}", format_output_handle(&handle, begin));
		}
	}
}
