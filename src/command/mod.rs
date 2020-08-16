extern crate colored;

use colored::*;

use std::time::Instant;

use crate::{config::Config, program::Program};

pub fn command_src(program: &Program) -> Result<String, &'static str> {
	Ok(format!(
		"{}:\n____________________________\n{}\n____________________________",
		"Source code".bold(),
		program.generate_source_code(true).italic()
	))
}

pub fn format_output_handle(handle: &std::process::Output, duration: Instant) -> String {
	let timing = format!(
		"{} \t{}",
		"[timing]".italic().dimmed(),
		format!("{:?}", duration.elapsed()).bold()
	);
	let status = format!(
		"{} \t{}",
		"[status]".italic().yellow(),
		handle.status.code().unwrap_or(-100)
	);
	let stderr = format!(
		"{} \t{}",
		"[stderr]".italic().red(),
		String::from_utf8_lossy(&handle.stderr).bold()
	);
	let stdout = format!(
		"{} \t{}",
		"[stdout]".italic().green(),
		String::from_utf8_lossy(&handle.stdout).bold()
	);
	format!("{}\n{}\n{}\n{}", timing, status, stderr, stdout)
}

pub fn command_del(input: &str, program: &mut Program) -> Result<String, &'static str> {
	let chunks: Vec<&str> = input.split_whitespace().collect();
	let index = match chunks[1].parse::<usize>() {
		Ok(index) => index,
		_ => return Err("Argument is not a number."),
	};

	let removed_statement;
	if index < program.includes.len() {
		removed_statement = program.includes[index].clone();
		program.includes.remove(index);
	} else if index < program.includes.len() + program.defines.len() {
		removed_statement = program.defines[index - program.includes.len()].clone();
		program.defines.remove(index - program.includes.len());
	} else if index < program.includes.len() + program.defines.len() + program.functions.len() {
		removed_statement =
			program.functions[index - program.includes.len() - program.defines.len()].clone();
		program
			.functions
			.remove(index - program.includes.len() - program.defines.len());
	} else if index
		< program.includes.len()
			+ program.defines.len()
			+ program.functions.len()
			+ program.statements.len()
	{
		removed_statement = program.statements
			[index - program.includes.len() - program.defines.len() - program.functions.len()]
		.clone();
		program.statements.remove(
			index - program.includes.len() - program.defines.len() - program.functions.len(),
		);
	} else {
		return Err("Statement index is out of range");
	}
	Ok(format!("removed [{}]", removed_statement))
}

pub fn command_run(program: &mut Program, c: &mut Config) -> Result<String, &'static str> {
	let begin = Instant::now();
	match program.run(c) {
		Ok(output) => Ok(format_output_handle(&output, begin)),
		Err(_) => Err("A Runtime error must have occured."),
	}
}

pub fn command_argv(input: &str, program: &mut Program) -> Result<String, &'static str> {
	if input.len() > 4 {
		let mut chunks: Vec<&str> = input.split_whitespace().collect();
		chunks.remove(0);
		let new_argv = chunks.join(" ");
		program.set_argv(new_argv);
		Ok(format!("new argv = [{}]", program.argv))
	} else {
		Ok(format!("current argv = [{}]", program.argv))
	}
}

pub fn command_xcc(input: &str, conf: &mut Config) -> Result<String, &'static str> {
	if input.len() > 4 {
		let chunks: Vec<&str> = input.split_whitespace().collect();
		let new_cc = chunks[1].to_string();
		conf.cc = new_cc;
		Ok(format!("new compiler = [{}]", conf.cc))
	} else {
		Ok(format!("current compiler = [{}]", conf.cc))
	}
}

pub fn execute_command(
	input: &str,
	program: &mut Program,
	c: &mut Config,
) -> Result<String, &'static str> {
	match &input[0..4] {
		"~src" => command_src(program),
		"~del" => command_del(input, program),
		"~run" => command_run(program, c),
		"~arg" => command_argv(input, program),
		"~xcc" => command_xcc(input, c),
		_ => Err("Command not found."),
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{common::create_dummy_program, program::StmsType};

	#[test]
	fn command_src_test() {
		let (mut p, mut _c) = create_dummy_program();
		p.push("int x = 10;", StmsType::Stmt);
		assert!(p.generate_source_code(false).len() > 10);
	}

	#[test]
	fn command_del_test() {
		let (mut p, mut _c) = create_dummy_program();
		p.push("int a = 10;", StmsType::Stmt);
		p.push(r#"printf("marker:%d\n", a)"#, StmsType::Stmt);
		match command_del("~del 1", &mut p) {
			Ok(_) => assert!(true),
			_ => assert!(false),
		};
	}

	#[test]
	fn command_argv_test() {
		let (mut p, mut _c) = create_dummy_program();
		p.push(
			r#"for (int i = 0; i < argc; i++) {printf("argv[%d] = %s\n", i, argv[i]);}"#,
			StmsType::Stmt,
		);
		match command_argv("~argv FOO BAR", &mut p) {
			Ok(_) => assert!(true),
			Err(_) => assert!(false),
		}
	}

	#[test]
	fn command_xcc_test() {
		let (_p, mut c) = create_dummy_program();
		match command_xcc("~xcc clang", &mut c) {
			Err(_) => assert!(false),
			_ => (),
		}
		assert!(c.cc == "clang");
	}
}
