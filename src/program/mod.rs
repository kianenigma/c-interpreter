use std::io::Write;
use std::process::{Command, Stdio};
use std::error::Error;
use std::fs::File;

pub enum StmsType {
  Stmt,
  Def,
  Inc
}

pub struct Program {
  pub statements: Vec<String>,
  pub defines: Vec<String>,
  pub includes: Vec<String>,
  pub last_push: StmsType
}

impl Program {
  pub fn populate_default(&mut self) {
    self.includes.push("#include <stdio.h>\n".to_owned());
  }

  pub fn push(&mut self, stmt: &str, stmt_type: StmsType) {
    match stmt_type {
      StmsType::Def => self.defines.push(String::from(stmt)),
      StmsType::Inc => self.includes.push(String::from(stmt)),
      StmsType::Stmt => self.statements.push(String::from(stmt))
    }
    self.last_push = stmt_type;
  }

  pub fn pop(&mut self) {
    match self.last_push {
      StmsType::Def => self.defines.pop(),
      StmsType::Inc => self.includes.pop(),
      StmsType::Stmt => self.statements.pop()
    };
  }

  pub fn generate_source_code(&self, verbose: bool) -> String{
    let mut source_includes = String::new();
    let mut source_defines = String::new();
    let mut source_statements = String::new();
    let mut counter = 0;
    
    for inc in &self.includes {
      if verbose { source_includes.push_str(&format!("({}){}\n", counter, inc)); } 
      else { source_includes.push_str(&format!("{}\n", inc)) }
      counter += 1;
    }

    for def in &self.defines {
      if verbose { source_defines.push_str(&format!("({}){}\n", counter, def)); } 
      else { source_defines.push_str(&format!("{}\n", def)); }
      counter += 1;
    }

    for stmt in &self.statements {
      if verbose { source_statements.push_str(&format!("\t({}){}\n", counter, stmt)); } 
      else { source_statements.push_str(&format!("\t{}\n", stmt)); } 
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

 pub fn run(&self) -> Result<std::process::Output, String> {
    let source = self.generate_source_code(false);
    // create temp file
    const TEMP_FILE: &'static str = "temp.c";
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
      return Err(format!("Compile Error: {}", compile_stderr));
    }

    // execute the binary 
    let child = match Command::new(String::from("./a.out"))
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .spawn() {
        Ok(child) => child,
        Err(why) => panic!("Failed to Execute: {}", why.description())
    };

    let handle = match child.wait_with_output() {
      Ok(handle) => handle,
      Err(why) => panic!("Failed to Execute: {}", why.description())
    };
    Ok(handle)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn create_dummy_program() -> Program {
    let mut p: Program = Program {
      defines: vec![], 
      includes: vec![], 
      statements: vec![], 
      last_push: StmsType::Stmt
    };
    p.populate_default();
    p.push("#include <stdlib.h>", StmsType::Inc);
    p.push("#define KB 1024", StmsType::Def);
    p.push("int init_value = 10;", StmsType::Stmt); 

    p
  }

  #[test]
  fn state_initial() {
    let mut p = create_dummy_program();

    assert_eq!(p.defines.len(), 1);
    assert_eq!(p.includes.len(), 2);
    assert_eq!(p.statements.len(), 1);
    p.push("int b = 20;", StmsType::Stmt);
    assert_eq!(p.statements.len(), 2);
  }

  #[test]
  fn state_add() {
    let mut p = create_dummy_program();
    p.push("#include <stddef.h>", StmsType::Inc);
    p.push("#include <stdint.h>", StmsType::Inc);
    p.push("#include <assert.h>", StmsType::Inc);

    p.push("#define X 10", StmsType::Def);
    p.push("#define XX 10", StmsType::Def);
    p.push("#define XXX 10", StmsType::Def);

    p.push("int x = 10;", StmsType::Stmt);
    p.push("int xx = 10;", StmsType::Stmt);
    p.push("int xxx = 10;", StmsType::Stmt);

    assert_eq!(p.defines.len(), 4);
    assert_eq!(p.includes.len(), 5);
    assert_eq!(p.statements.len(), 4);
  }

  #[test]
  fn run_basic() {
    let p = create_dummy_program();
    let handle = p.run();
    let handle1 = p.run();
    // TODO: why handle is being borrowed here despite being a ref? Guess because of unwrap? 
    assert_eq!(String::from_utf8_lossy(&handle.unwrap().stdout), "Hello C-Interpreter!\n");
    assert_eq!(String::from_utf8_lossy(&handle1.unwrap().stderr), "");
  }

  #[test]
  fn run_compile_error() {
    let mut p = create_dummy_program();
    p.push("int x = 10", StmsType::Stmt);
    assert_ne!(p.run().unwrap_err(), "");
  }

  #[test]
  fn run_runtime_error() {
    let mut p = create_dummy_program();
    p.push("int b = 0;", StmsType::Stmt);
    p.push("int x = 10/b;", StmsType::Stmt);
    assert_eq!(p.run().unwrap().status.code().unwrap_or_else(|| 1000), 1000);
  }
}