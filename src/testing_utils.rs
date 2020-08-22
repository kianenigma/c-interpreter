#![cfg(test)]

use crate::{
	config::Config,
	program::{Program, StatementType},
};

pub fn create_dummy_program() -> (Program, Config) {
	let mut p: Program = Default::default();
	p.push("#include <stdlib.h>", StatementType::Inc);
	p.push("#define KB 1024", StatementType::Def);
	p.push("int init_value = 10;", StatementType::Stmt);

	let c: Config = Config {
		cc: "gcc".to_string(),
	};

	(p, c)
}
