extern crate colored; 

use std::io; 
use std::io::Write;
use std::process::Command;
use std::error::Error;
use std::fs::File;

use colored::*;

enum StmsType {
    Stmt,
    Def,
    Inc
}

struct Program {
  statements: Vec<String>,
  defines: Vec<String>,
  includes: Vec<String>,
  last_push: StmsType
}

impl Program {
  fn populate_default(&mut self) {
    self.includes.push("#include <stdio.h>\n".to_owned());
  }

  fn push(&mut self, stmt: &str, stmtType: StmsType) {
    match stmtType {
      StmsType::Def => self.defines.push(String::from(stmt)),
      StmsType::Inc => self.includes.push(String::from(stmt)),
      StmsType::Stmt => self.statements.push(String::from(stmt))
    }
    self.last_push = stmtType;
  }

  fn pop(&mut self) {
    match self.last_push {
      StmsType::Def => self.defines.pop(),
      StmsType::Inc => self.includes.pop(),
      StmsType::Stmt => self.statements.pop()
    };
  }

  fn generate_source_code(&self, verbose: bool) -> String{
    let mut source_includes = String::new();
    let mut source_defines = String::new();
    let mut source_statements = String::new();
    let mut counter = 0;
    
    for inc in &self.includes {
      if verbose { source_includes.push_str(&format!("({}){}", counter, inc)); } 
      else { source_includes.push_str(&format!("{}", inc)) }
      counter += 1;
    }

    for def in &self.defines {
      if verbose { source_defines.push_str(&format!("({}){}", counter, def)); } 
      else { source_defines.push_str(&format!("{}", def)); }
      counter += 1;
    }

    for stmt in &self.statements {
      if verbose { source_statements.push_str(&format!("\t({}){}", counter, stmt)); } 
      else { source_statements.push_str(&format!("\t{}", stmt)); } 
      counter += 1;
    }

  format!(
    r#"
{includes}

{defines}

int main() {{
// statements 
{statements}

printf("Hello C-Interpreter!\n");
return 0;
}}"#, includes = source_includes, defines = source_defines, statements = source_statements)
  }

}

fn main() {
  let mut program = Program {
    defines: vec![], 
    includes: vec![], 
    statements: vec![], 
    last_push: StmsType::Stmt
  };
  program.populate_default();

  loop {
    print_prompt();
    let current_statement = match read_input() {
        Ok(input) => input,
        Err(_err) => { println!("-- Unable to read input: {}", _err); continue; }
    };

    match execute_command(&current_statement, &mut program) {
      Ok(_) => { continue; },
      Err(_) => ()
    };

    let stmtType: StmsType = get_statement_type(&current_statement);    
    program.push(&current_statement, stmtType); 

    let c_program = program.generate_source_code(false);
    match compile_execute(&c_program) {
      Err(why) => { 
        println!("{}: {}", "--- Error".red(), why);
        program.pop();
      },
      Ok(handle) => {
        println!("{} {}", "[stderr]---".red(), String::from_utf8_lossy(&handle.stderr).bold());
        println!("{} {}", "[stdout]+++".green(), String::from_utf8_lossy(&handle.stdout).bold());
      }
    }
  }
}


fn print_prompt() {
  const PROMPT: &str = ">>> ";
  print!("{}", PROMPT);
  std::io::stdout().flush().unwrap();
}

fn read_input() -> Result<String, &'static str> {
  let mut input_buffer = String::new();
  match io::stdin().read_line(&mut input_buffer) {
    Ok(_) => { Ok(input_buffer) }
    Err(_) => { Err("Unable to parse input") }
  }
}

fn command_src(input: &str, program: &Program) -> Result<(), ()> {
  println!("{}:\n_____{}_____", "Source code".bold(), program.generate_source_code(true).italic());
  Ok(())
}

fn command_del(input: &str, program: &mut Program) -> Result<(), ()> {
  let chunks: Vec<&str> = input.split_whitespace().collect();

  let index = match chunks[1].parse::<usize>() {
    Ok(index) => index,
    _ => return Err(())
  };

  if index < program.includes.len() {
    program.includes.remove(index);
  }
  else if index < program.defines.len() {
    program.defines.remove(index - program.includes.len());
  }
  else if index < program.statements.len() {
    program.statements.remove(index - program.includes.len() - program.defines.len());
  }
  else {
    return Err(())
  }
  Ok(())
}

fn execute_command(input: &str, program: &mut Program) -> Result<(), ()> {
  if &input[0..1] != "~" {
     return Err(());
  }

  match &input[0..4] {
      "~src" => command_src(input, program),
      "~del" => command_del(input, program),
      _ => Err(())
  }
}

fn get_statement_type(stms: &str) -> StmsType {
  match &stms[0..4] {
      "#inc" => StmsType::Inc,
      "#def" => StmsType::Def,
      _ => StmsType::Stmt
  }
}

fn compile_execute(source: &String) -> Result<std::process::Output, String> {
  // create temp file
  const TEMP_FILE: &'static str = "temp.c";
  const BINARY_FILE: &'static str = "temp.bin";
  const CC: &'static str = "gcc";

  let mut temp_source_file = match File::create(TEMP_FILE) {
    Err(why) => panic!("Could not create temp file [{}]", why.description()),
    Ok(file) => file
  }; 

  // write source to a temp file
  match temp_source_file.write_all(source.as_bytes()) {
      Err(why) => panic!("Could not write to temp file: [{}]", why.description()),
      Ok(_) => ()
  }

  // spawn a compiler 
  let compile_handle = match Command::new(CC).arg(TEMP_FILE).output() {
    Err(why) => panic!("Failed to compile: {}", why.description()),
    Ok(handle) => handle
  };

  let compile_stderr = String::from_utf8_lossy(&compile_handle.stderr);
  if compile_stderr.len() > 0 {
    return Err(format!("++ Compile Error: {}", compile_stderr));
  }

  // execute the binary 
  let handle = match Command::new(format!("./a.out")).output() {
      Err(why) => panic!("Failed to execute : {}", why.description()),
      Ok(handle) => handle
  };

  Ok(handle)
}

#[test]
fn basic() {
    let source = String::from(r#"#include <stdio.h>
    int main() {int a = 10; printf("hello world! [%d]", a); return 0;}"#); 

    let handle = compile_execute(&source);
    print!("++ stdout is {:?}", handle.stdout);
    print!("++ stderr is {:?}", handle.stderr);
}