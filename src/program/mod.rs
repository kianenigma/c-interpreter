use std::{
	error::Error,
	fs::File,
	io::Write,
	process::{Command, Stdio},
};

use crate::constants::TEMP_FILE;

use crate::config::Config;

pub enum StatementType {
	Stmt,
	Def,
	Inc,
	Func,
}

impl<'a, T: Into<&'a str>> From<T> for StatementType {
	fn from(t: T) -> Self {
		let string: &str = t.into();
		match &string[0..4] {
			"#inc" => StatementType::Inc,
			"#def" => StatementType::Def,
			"#fun" => StatementType::Func,
			_ => StatementType::Stmt,
		}
	}
}

impl Default for StatementType {
	fn default() -> Self {
		Self::Stmt
	}
}

pub struct Program {
	pub statements: Vec<String>,
	pub defines: Vec<String>,
	pub includes: Vec<String>,
	pub functions: Vec<String>,
	pub last_push: StatementType,
	pub argv: String,
}

impl Default for Program {
	fn default() -> Self {
		Self {
			includes: vec!["#include <stdio.h>\n".to_owned()],
			functions: vec![r#"void test() { printf("Hello C-Interpreter!\n"); }"#.to_owned()],
			statements: Default::default(),
			defines: Default::default(),
			argv: Default::default(),
			last_push: Default::default(),
		}
	}
}

impl Program {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}

	pub fn push(&mut self, stmt: &str, stmt_type: StatementType) {
		match stmt_type {
			StatementType::Def => self.defines.push(String::from(stmt)),
			StatementType::Inc => self.includes.push(String::from(stmt)),
			StatementType::Stmt => self.statements.push(String::from(stmt)),
			StatementType::Func => self.functions.push(String::from(&stmt[4..])),
		}
		self.last_push = stmt_type;
	}

	pub fn pop(&mut self) {
		match self.last_push {
			StatementType::Def => self.defines.pop(),
			StatementType::Inc => self.includes.pop(),
			StatementType::Stmt => self.statements.pop(),
			StatementType::Func => self.functions.pop(),
		};
	}

	pub fn set_argv(&mut self, argv: String) {
		self.argv = argv;
	}

	pub fn generate_source_code(&self, verbose: bool) -> String {
		let mut source_includes = String::new();
		let mut source_defines = String::new();
		let mut source_statements = String::new();
		let mut source_functions = String::new();
		let mut counter = 0;

		for inc in &self.includes {
			if verbose {
				source_includes.push_str(&format!("({}){}\n", counter, inc));
			} else {
				source_includes.push_str(&format!("{}\n", inc))
			}
			counter += 1;
		}

		for def in &self.defines {
			if verbose {
				source_defines.push_str(&format!("({}){}\n", counter, def));
			} else {
				source_defines.push_str(&format!("{}\n", def));
			}
			counter += 1;
		}

		for func in &self.functions {
			if verbose {
				source_functions.push_str(&format!("({}){}\n", counter, func));
			} else {
				source_functions.push_str(&format!("{}\n", func));
			}
			counter += 1;
		}

		for stmt in &self.statements {
			if verbose {
				source_statements.push_str(&format!("\t({}){}\n", counter, stmt));
			} else {
				source_statements.push_str(&format!("\t{}\n", stmt));
			}
			counter += 1;
		}

		format!(
			r#"{includes}

{defines}

{functions}

int main(int argc, char **argv) {{
{statements}

    return 0;
}}"#,
			includes = source_includes,
			defines = source_defines,
			functions = source_functions,
			statements = source_statements
		)
	}

	pub fn run(&self, config: &Config) -> Result<std::process::Output, String> {
		let source = self.generate_source_code(false);

		// create temp file
		let mut temp_source_file = match File::create(TEMP_FILE) {
			Err(why) => panic!("Could not create temp file [{}]", why.description()),
			Ok(file) => file,
		};

		// write source to a temp file
		match temp_source_file.write_all(source.as_bytes()) {
			Err(why) => panic!("Could not write to temp file: [{}]", why.description()),
			Ok(_) => (),
		}

		// spawn a compiler
		let cc = config.cc.clone();
		let compile_handle = match Command::new(cc).arg(TEMP_FILE).output() {
			Err(why) => panic!("Failed spawn compiler: {}", why.description()),
			Ok(handle) => handle,
		};

		let compile_stderr = String::from_utf8_lossy(&compile_handle.stderr);
		if compile_stderr.len() > 0 && compile_stderr.contains("error:") {
			return Err(format!("Compile Error:\n {}", compile_stderr));
		}

		// execute the binary
		let args: Vec<&str> = self.argv.split_whitespace().collect();
		let child = match Command::new(String::from("./a.out"))
			.args(args)
			.stdout(Stdio::piped())
			.stderr(Stdio::piped())
			.spawn()
		{
			Ok(child) => child,
			Err(why) => panic!("Failed to Execute: {}", why.description()),
		};

		let handle = match child.wait_with_output() {
			Ok(handle) => handle,
			Err(why) => panic!("Failed to Execute: {}", why.description()),
		};
		Ok(handle)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::testing_utils::create_dummy_program;

	#[test]
	fn state_initial() {
		let (mut p, mut _c) = create_dummy_program();

		assert_eq!(p.defines.len(), 1);
		assert_eq!(p.includes.len(), 2);
		assert_eq!(p.statements.len(), 1);
		p.push("int b = 20;", StatementType::Stmt);
		assert_eq!(p.statements.len(), 2);
	}

	#[test]
	fn state_add() {
		let (mut p, mut _c) = create_dummy_program();
		p.push("#include <stddef.h>", StatementType::Inc);
		p.push("#include <stdint.h>", StatementType::Inc);
		p.push("#include <assert.h>", StatementType::Inc);

		p.push("#define X 10", StatementType::Def);
		p.push("#define XX 10", StatementType::Def);
		p.push("#define XXX 10", StatementType::Def);

		p.push("int x = 10;", StatementType::Stmt);
		p.push("int xx = 10;", StatementType::Stmt);
		p.push("int xxx = 10;", StatementType::Stmt);

		assert_eq!(p.defines.len(), 4);
		assert_eq!(p.includes.len(), 5);
		assert_eq!(p.statements.len(), 4);
	}

	#[test]
	fn run_basic() {
		let (mut p, c) = create_dummy_program();
		p.push("test();", StatementType::Stmt);
		let handle = p.run(&c);
		let handle1 = p.run(&c);
		// TODO: why handle is being borrowed here despite being a ref? Guess because of unwrap?
		assert_eq!(
			String::from_utf8_lossy(&handle.unwrap().stdout),
			"Hello C-Interpreter!\n"
		);
		assert_eq!(String::from_utf8_lossy(&handle1.unwrap().stderr), "");
	}

	#[test]
	fn argv() {
		let (mut p, c) = create_dummy_program();
		p.push(
			r#"for (int i = 0; i < argc; i++) {printf("argv[%d] = %s\n", i, argv[i]);}"#,
			StatementType::Stmt,
		);
		let handle = p.run(&c);
		assert!(String::from_utf8_lossy(&handle.unwrap().stdout).contains("argv[0]"));
	}

	#[test]
	fn functions() {
		let (mut p, c) = create_dummy_program();
		p.push(
			r#"#fun void foo() { printf("MARKER"); }"#,
			StatementType::Func,
		);
		p.push("foo();", StatementType::Stmt);
		let handle = p.run(&c);
		assert!(String::from_utf8_lossy(&handle.unwrap().stdout).contains("MARKER"));
	}

	#[test]
	fn run_compile_error() {
		let (mut p, c) = create_dummy_program();
		p.push("int x = 10", StatementType::Stmt);
		assert_ne!(p.run(&c).unwrap_err(), "");
	}

	#[test]
	fn run_runtime_error() {
		let (mut p, c) = create_dummy_program();
		p.push("int b = 0;", StatementType::Stmt);
		p.push("int x = 10/b;", StatementType::Stmt);
		// println!("{:?}", p.run(&c));
		assert_eq!(
			p.run(&c).unwrap().status.code().unwrap_or_else(|| 1000),
			1000
		);
	}

	#[test]
	fn compiler_warnings_ignored() {
		let (mut p, c) = create_dummy_program();
		p.includes.remove(0);
		assert!(p.run(&c).unwrap().stderr.len() == 0);
	}
}
