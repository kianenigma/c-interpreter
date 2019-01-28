use crate::program::{Program, StmsType};
use crate::config::Config;

pub fn create_dummy_program() -> (Program, Config) {
    let mut p: Program = Program {
      defines: vec![], 
      includes: vec![], 
      functions: vec![], 
      statements: vec![], 
      last_push: StmsType::Stmt,
      argv: String::from("")
    };
    p.populate_default();
    p.push("#include <stdlib.h>", StmsType::Inc);
    p.push("#define KB 1024", StmsType::Def);
    p.push("int init_value = 10;", StmsType::Stmt); 

    let c: Config = Config { cc: "gcc".to_string() };

    (p, c)
  }