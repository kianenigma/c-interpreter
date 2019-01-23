extern crate colored; 

use colored::*;

use std::time::{Instant};

use crate::program::Program;

pub fn command_src(program: &Program) -> Result<(), ()> {
  println!("{}:\n_____{}_____", "Source code".bold(), program.generate_source_code(true).italic());
  Ok(())
}

pub fn print_output_handle(handle: &std::process::Output, duration: Instant) {
  println!("{} \t{}", "[timing]".italic().dimmed(), format!("{:?}", duration.elapsed()).bold());
  println!("{} \t{}", "[status]".italic().yellow(), handle.status.code().unwrap());
  println!("{} \t{}", "[stderr]".italic().red(), String::from_utf8_lossy(&handle.stderr).bold());
  println!("{} \t{}", "[stdout]".italic().green(), String::from_utf8_lossy(&handle.stdout).bold());
}

pub fn command_del(input: &str, program: &mut Program) -> Result<(), ()> {
  let chunks: Vec<&str> = input.split_whitespace().collect();

  let index = match chunks[1].parse::<usize>() {
    Ok(index) => index,
    _ => return Err(())
  };

  if index < program.includes.len() {
    program.includes.remove(index);
  }
  else if index < program.includes.len() + program.defines.len() {
    program.defines.remove(index - program.includes.len());
  }
  else if index < program.includes.len() + program.defines.len() + program.statements.len() {
    program.statements.remove(index - program.includes.len() - program.defines.len());
  }
  else {
    return Err(())
  }
  Ok(())
}

pub fn command_run(program: &mut Program) -> Result<(), ()> {
  let begin = Instant::now();
  print_output_handle(&program.run().unwrap(), begin);
  Ok(())
}

pub fn command_argv<'a>(input: &'a str, program: &'a mut Program) -> Result<(), ()> {
  if input.len() > 4 {
    let mut chunks: Vec<&str> = input.split_whitespace().collect();
    chunks.remove(0);
    let new_argv = chunks.join(" ");
    program.set_argv(new_argv);
    println!("new argv = [{}]", program.argv);
  } 
  else {
    println!("current argv = [{}]", program.argv);
  }
  Ok(())
}

pub fn execute_command(input: &str, program: &mut Program) -> Result<(), ()> {
  if &input[0..1] != "~" {
     return Err(());
  }

  match &input[0..4] {
      "~src" => command_src(program),
      "~del" => command_del(input, program),
      "~run" => command_run(program),
      "~arg" => command_argv(input, program),
      _ => Err(())
  }
}

#[cfg(test)] 
mod test {
  use super::*; 
  use crate::program::StmsType;
  
  fn create_dummy_program<'a>() -> Program {
    let mut p: Program = Program {
      defines: vec![],
      includes: vec![],
      statements: vec![],
      functions: vec![],
      last_push: StmsType::Stmt,
      argv: String::from("")
    };
    p.populate_default();
    p.push("#include <stdlib.h>", StmsType::Inc);
    p.push("#define KB 1024", StmsType::Def);
    p.push("int init_value = 10;", StmsType::Stmt); 

    p
  }

  #[test]
  fn command_src_test() {
    let mut p = create_dummy_program();
    p.push("int x = 10;", StmsType::Stmt);
    assert!(p.generate_source_code(false).len() > 10);
  }

  #[test]
  fn command_del_test() {
    let mut p = create_dummy_program();
    p.push("int a = 10;", StmsType::Stmt);
    p.push(r#"printf("marker:%d\n", a)"#, StmsType::Stmt);
    match command_del("~del 1", &mut p) {
      Ok(_) => assert!(true),
      _ => assert!(false)
    };
  }

  #[test]
  fn command_argv_test() {
      let mut p = create_dummy_program();
      p.push(r#"for (int i = 0; i < argc; i++) {printf("argv[%d] = %s\n", i, argv[i]);}"#, StmsType::Stmt);
      match command_argv("~argv FOO BAR", &mut p) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false)
      }
  }
}