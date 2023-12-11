// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::ir::frontend;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(#[allow(clippy::all)]#[allow(clippy::uninlined_format_args)]pub parser,"/ir/frontend/ir.rs");

pub fn compile(input: &str) -> Box<frontend::Module> {
    parser::ModParser::new().parse(input).unwrap()
}

#[cfg(test)]
mod parser_test {
    use crate::ir::{
        context::IRContext,
        frontend::{parser::compile, translate::translate_main_module},
        printer::IRPrinter,
    };

    #[test]
    fn parse_type_def() {
        let module = compile(
            r#"
module_name = "test"
type struct.hello1 = { a: i8*, a: i8**, c: {i32: i16}, }
type hello2 = i8*
type hello3 = [u8]
type hello4 = { u16: u16 }
"#,
        );
        let mut ctx = IRContext::default();
        translate_main_module(&mut ctx, &module);
        let mut p = IRPrinter::new(&ctx);
        let mut w = String::new();
        p.print_main_module(&mut w).unwrap();
        println!("{}", w)
    }

    #[test]
    fn parse_func_def() {
        let module = compile(
            r#"
module_name = "test"
fn foo( %0: i8, %1: i8, %2: [u8;10], ) -> i8 {
    0:
        let %3: i8 = add(add(%0: i8, %1: i8, ), %1: i8, )
        %3 = sub(%3: i8, %2: i8, )
        let %4: i8 = add(%3: i8, 2: i8, )
        let %5: str = "\u{4F60}\x52": str
        call(@ir.context.call.sender(2: i8, ), )
        br(bb 1, )
       
    1:
        %6 = eq(1: i32, 1: i32, )
        br_if(%5: bool, bb 0, bb 2, )
    2:
        ret(%3: i8, )
}


"#,
        );
        let mut ctx = IRContext::default();
        translate_main_module(&mut ctx, &module);
        let mut p = IRPrinter::new(&ctx);
        let mut w = String::new();
        p.print_main_module(&mut w).unwrap();
        println!("{}", w)
    }
    #[test]
    fn parse_match() {
        let module = compile(
            r#"
module_name = "test"
type struct.MatchContract.Dog = {color: str, }
type struct.MatchContract.Cat = {color: str, }
type struct.MatchContract.Rabbit = {size: u32, }
type enum.MatchContract.Color = {Red: void, Yellow: void, Blue: void, }
type enum.MatchContract.Animal = {Dog: %struct.MatchContract.Dog*, 
                                  Cat: %struct.MatchContract.Cat*,
                                  Rabbit: %struct.MatchContract.Rabbit*, }
contract SimpleStorage {
  state {foo : %struct.MatchContract.Rabbit*, bar: i32, }
	pub fn simple_match() !ir_ir_debug_location !0 {
  	0:
      let %0 : %enum.MatchContract.Color* !ir_ir_debug_location !0 = malloc(%enum.MatchContract.Color, )
      set_field(%0: %enum.MatchContract.Color*, 0: i32, 0: i32, ) !ir_ir_debug_location !0 
      let %1 : i32 !ir_ir_debug_location !0 = get_field(%0: %enum.MatchContract.Color*, 0: i32, ) -> i32
      match(%1: i32, bb 1, 0: i32, bb 1, 
                           1: i32, bb 2, 
                           2: i32, bb 3, ) 
    1:
      br(bb 4, )
    2:
      br(bb 4, )
    3:
      br(bb 4, )
  	4:
     match(%1: i32, bb 5,0: i32, bb 6, )
    5:
      br(bb 7, )
    6:
      br(bb 7, )
    7:
      let %2 : %enum.MatchContract.Animal* = alloca(%enum.MatchContract.Animal, )
      let %3 : %struct.MatchContract.Dog* = alloca(%struct.MatchContract.Dog, )
      set_field(%3: %struct.MatchContract.Dog*, "red": str, 0: i32, )
      set_field(%2: %enum.MatchContract.Animal*, 0: i32, 0: i32, )
      set_field(%2: %enum.MatchContract.Animal*, %3: %struct.MatchContract.Dog*,
        1: i32, )
      match(%1: i32, bb 8,0: i32, bb 9, )
    8:
      br(bb 10, )
    9:
      let %4 : %struct.MatchContract.Dog* = get_field(%3: %enum.MatchContract.Animal*, 1: i32, ) 
            -> %struct.MatchContract.Dog*
      br(bb 10, )
    10:
      free(%0 : %enum.MatchContract.Color*, )
      ret()
  }
}

meta !0 = !{2: u64, 5: u64, "/Users/admin/ir/test.ir": str, }

"#,
        );
        let mut ctx = IRContext::default();
        translate_main_module(&mut ctx, &module);
        let mut p = IRPrinter::new(&ctx);
        let mut w = String::new();
        p.print_main_module(&mut w).unwrap();
        println!("{}", w);
    }

    #[test]
    fn parse_int_cast() {
        let module = compile(
            r#"
module_name = "test"

contract SimpleCast {
	pub fn simple_cast()  {
  	0:
      let %0: u16 = int_cast(1: u8, ) -> u16
      let %1: u32 = int_cast(1: u16, ) -> u32
      let %2: u64 = int_cast(1: u32, ) -> u64
      let %3: u32 = int_cast(1: u64, ) -> u128
      let %4: i16 = int_cast(-1: i8, ) -> i16
      let %5: i32 = int_cast(-1: i16, ) -> i32
      let %6: i64 = int_cast(-1: i32, ) -> i64
      let %7: i128 = int_cast(-1: i64, ) -> i128

      let %8 : u8 = int_cast(1: u128, ) -> u8
      let %9 : i8 = int_cast(-1: i128, ) -> i8


      ret()
  }
}

"#,
        );
        let mut ctx = IRContext::default();
        translate_main_module(&mut ctx, &module);
        let mut p = IRPrinter::new(&ctx);
        let mut w = String::new();
        p.print_main_module(&mut w).unwrap();
        println!("{}", w);
    }

    #[test]
    fn parse_binary_instr() {
        let module = compile(
            r#"
module_name = "test"

contract SimpleBinary {
	pub fn simple_binary()  {
  	0:
      let %0: i32 = add(1: i32, 2: i32, )
      %0 = sub(%0: i32, 3: i32, )
      %0 = mul(%0: i32, 4: i32, )
      %0 = div(%0: i32, 5: i32, )
      %0 = mod(%0: i32, 6: i32, )
      %0 = exp(%0: i32, 7: i32, )
      %0 = bit_and(%0: i32, 8: i32, )
      %0 = bit_or(%0: i32, 9: i32, )
      %0 = bit_xor(%0: i32, 10: i32, )

      let %1: bool = false: bool
      %1 = and(%1: bool, true: bool, )
      %1 = or(%1: bool, true: bool, )


      
      ret()
  }
}

"#,
        );
        let mut ctx = IRContext::default();
        translate_main_module(&mut ctx, &module);
        let mut p = IRPrinter::new(&ctx);
        let mut w = String::new();
        p.print_main_module(&mut w).unwrap();
        println!("{}", w);
    }

    #[test]
    fn parse_unary_instr() {
        let module = compile(
            r#"
module_name = "test"

contract SimpleUnary {
	pub fn simple_unary()  {
  	0:
      let %0: i32 = 1: i32
      %0 = bit_not(%0: i32, )
      
      let %1: bool = false: bool
      %1 = not(%1: bool, )
      
      ret()
  }
}

"#,
        );
        let mut ctx = IRContext::default();
        translate_main_module(&mut ctx, &module);
        let mut p = IRPrinter::new(&ctx);
        let mut w = String::new();
        p.print_main_module(&mut w).unwrap();
        println!("{}", w);
    }

    #[test]
    fn parse_storage_instr() {
        let module = compile(
            r#"
module_name = "Test"
contract SimpleStorage {
    state {
        a: str,
    }

    pub fn simple_storage(%0: str, )  {
        1:
            let %1: str = storage_load(get_storage_path("a": str, ) !mutable_path_index !0 , ) -> str 
            storage_store(get_storage_path("a": str, ) !mutable_path_index !1 , %0: str, )
            %1 = storage_load(get_storage_path("a": str, ) !mutable_path_index !0 , ) -> str 
            ret() 
    }

}
meta !0 = !{1: u32, }
meta !1 = !{1: u32, }
"#,
        );
        let mut ctx = IRContext::default();
        translate_main_module(&mut ctx, &module);
        let mut p = IRPrinter::new(&ctx);
        let mut w = String::new();
        p.print_main_module(&mut w).unwrap();
        println!("{}", w);
    }

    #[test]
    fn parse_cmp_instr() {
        let module = compile(
            r#"
module_name = "test"

contract SimpleCmp {
	pub fn simple_cmp()  {
  	0:
      let %0: bool = eq(1: i32, 2: i32, )
      let %1: bool = ne(1: i32, 2: i32, )
      let %2: bool = lt(1: i32, 2: i32, )
      let %3: bool = le(1: i32, 2: i32, )
      let %4: bool = gt(1: i32, 2: i32, )
      let %5: bool = ge(1: i32, 2: i32, )


      
      ret()
  }
}

"#,
        );
        let mut ctx = IRContext::default();
        translate_main_module(&mut ctx, &module);
        let mut p = IRPrinter::new(&ctx);
        let mut w = String::new();
        p.print_main_module(&mut w).unwrap();
        println!("{}", w);
    }
}
