// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use context::Yul2IRContext;
use lalrpop_util::lalrpop_mod;
use smart_ir::ir::printer::IRPrinter;

lalrpop_mod!(pub yul); // synthesized by LALRPOP
pub mod ast;
pub mod context;
pub mod instruction;
#[cfg(test)]
pub mod test;
pub mod transform;

pub fn yul2ir(src: &str, output: Option<&str>) -> Option<Yul2IRContext> {
    let block = match yul::ObjectParser::new().parse(&src){
        Ok(obj) => obj,
        Err(_) => return None,
    };
    let mut context = Yul2IRContext::new_with_object(block);
    match context.transform() {
        Ok(_) => {
            let mut p = IRPrinter::new(&context.ir_context);
            let mut w = String::new();
            p.print_modules(&mut w).unwrap();
            match output {
                Some(output_path) => {
                    std::fs::write(output_path, w).expect("File writeError");
                }
                None => {},
            }
            Some(context)
        }
        Err(e) => {
            println!("{:?}", e);
            return None;
        }
    }
}
