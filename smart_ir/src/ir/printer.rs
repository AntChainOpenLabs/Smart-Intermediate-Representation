// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use indexmap::IndexMap;

use super::builder::{IdentifierId, MetaDataId};
use super::cfg::{
    BasicBlock, CmpOp, Contract, ControlFlowGraph, Expr, Field, FunctionDefinition, Instr,
    InstrDescription, IntLiteral, IntType, Literal, MetaData, MetaDataNode, Module, PrimitiveType,
    Type, TypeDefinition,
};
use crate::ir::context::IRContext;
use std::fmt::Result;
use std::fmt::Write;

use crate::ir::cfg::{IR_MAP_ITER_TY, IR_PARAMPACK_TY, IR_STORAGE_PATH_TY, IR_VECTOR_ITER_TY};
pub struct IRPrinter<'ctx> {
    ctx: &'ctx IRContext,
    margin: usize,
}

impl<'ctx> IRPrinter<'ctx> {
    pub fn new(ctx: &'ctx IRContext) -> Self {
        Self { ctx, margin: 0 }
    }
}

impl IRPrinter<'_> {
    pub fn print_main_module(&mut self, w: &mut dyn Write) -> Result {
        let main_module = self.ctx.main_module.as_str();
        let modules = self.ctx.modules.borrow();
        let main_module = modules.get(main_module).unwrap();
        self.print_module(main_module, w)?;
        {
            let metadata = self.ctx.metadata.borrow();
            for (id, md_def) in metadata.iter() {
                self.print_md_def(*id, md_def, w)?;
            }
        }
        writeln!(w)
    }

    pub fn print_modules(&mut self, w: &mut dyn Write) -> Result {
        let main_module = self.ctx.main_module.as_str();
        let modules = self.ctx.modules.borrow();
        let main_module = modules.get(main_module).unwrap();
        self.print_module(main_module, w)?;
        for (name, module) in self.ctx.modules.borrow().iter() {
            if name == &self.ctx.main_module {
                continue;
            }
            self.print_module(module, w)?
        }
        {
            let metadata = self.ctx.metadata.borrow();
            for (id, md_def) in metadata.iter() {
                self.print_md_def(*id, md_def, w)?;
            }
        }
        writeln!(w)
    }

    pub fn print_module(&mut self, module: &Module, w: &mut dyn Write) -> Result {
        writeln!(w, "module_name = \"{}\"", module.name)?;

        for (_, ty_def) in module.types.iter() {
            self.print_ty_def(ty_def, w)?;
            writeln!(w)?;
        }

        for (_, func_def) in module.functions.iter() {
            self.print_func_def(func_def, w)?;
            writeln!(w)?;
        }

        if let Some(ref cont) = module.contract {
            self.print_cont(cont, w)?;
        }
        Ok(())
    }

    fn print_margin(&self, w: &mut dyn Write) -> Result {
        for _ in 0..self.margin {
            write!(w, " ")?;
        }
        Ok(())
    }

    fn print_ty_def(&mut self, def: &TypeDefinition, w: &mut dyn Write) -> Result {
        write!(w, "type {} = ", def.name)?;
        self.print_ty(&def.ty, w)?;
        self.print_metadatas(def, w)
    }

    fn print_md_def(&mut self, id: MetaDataId, def: &MetaData, w: &mut dyn Write) -> Result {
        write!(w, "meta !{id} = !{{")?;
        for lit in def.data.iter() {
            self.print_literal(lit, w)?;
            write!(w, ", ")?;
        }
        writeln!(w, "}}")
    }

    fn print_metadatas(&mut self, node: &dyn MetaDataNode, w: &mut dyn Write) -> Result {
        let metadatas = node.get_metadata();
        for (name, id) in metadatas.iter() {
            write!(w, " !{name} !{id}")?;
        }
        write!(w, " ")
    }

    fn print_ty(&mut self, ty: &Type, w: &mut dyn Write) -> Result {
        match ty {
            Type::Primitive(pri_ty) => match pri_ty {
                PrimitiveType::Str => write!(w, "str"),
                PrimitiveType::Bool => write!(w, "bool"),
                PrimitiveType::Void => write!(w, "void"),
                PrimitiveType::Int(int_ty) => match int_ty {
                    IntType::I8 => write!(w, "i8"),
                    IntType::I16 => write!(w, "i16"),
                    IntType::I32 => write!(w, "i32"),
                    IntType::I64 => write!(w, "i64"),
                    IntType::I128 => write!(w, "i128"),
                    IntType::I256 => write!(w, "i256"),
                    IntType::U8 => write!(w, "u8"),
                    IntType::U16 => write!(w, "u16"),
                    IntType::U32 => write!(w, "u32"),
                    IntType::U64 => write!(w, "u64"),
                    IntType::U128 => write!(w, "u128"),
                    IntType::U256 => write!(w, "u256"),
                },
            },
            Type::Map { key, value } => {
                write!(w, "{{")?;
                self.print_ty(key, w)?;
                write!(w, ": ")?;
                self.print_ty(value, w)?;
                write!(w, "}}")
            }
            Type::Array { elem, len } => {
                write!(w, "[")?;
                self.print_ty(elem, w)?;
                if let Some(len) = len {
                    write!(w, "; {len}")?;
                }
                write!(w, "]")
            }
            Type::Compound(fields) => {
                write!(w, "{{")?;
                for field in fields.iter() {
                    self.print_field(field, w)?;
                    write!(w, ", ")?;
                }
                write!(w, "}}")
            }
            Type::Tuple(elements) => {
                write!(w, "(")?;
                for ele in elements.iter() {
                    self.print_ty(ele, w)?;
                    write!(w, ", ")?;
                }
                write!(w, ")")
            }
            Type::Pointer(ptr) => {
                self.print_ty(ptr.as_ref(), w)?;
                write!(w, "*")
            }
            Type::Def(def) => {
                write!(w, "%{}", def.name)
            }
            Type::Builtin(ty) => match ty {
                super::cfg::BuiltinType::VectorIter => write!(w, "%{IR_VECTOR_ITER_TY}"),
                super::cfg::BuiltinType::MapIter => write!(w, "%{IR_MAP_ITER_TY}"),
                super::cfg::BuiltinType::Parampack => write!(w, "%{IR_PARAMPACK_TY}"),
                super::cfg::BuiltinType::StoragePath => write!(w, "%{IR_STORAGE_PATH_TY}"),
            },
        }
    }

    fn print_field(&mut self, field: &Field, w: &mut dyn Write) -> Result {
        write!(w, "{}: ", field.name)?;
        self.print_ty(&field.ty, w)
    }

    fn print_func_def(&mut self, def: &FunctionDefinition, w: &mut dyn Write) -> Result {
        self.print_margin(w)?;
        if def.is_external {
            write!(w, "pub ")?;
        }
        write!(w, "fn {}(", def.name)?;
        for (id, ty) in def.params.iter().enumerate() {
            write!(w, "%{id}: ")?;
            self.print_ty(ty, w)?;
            write!(w, ", ")?;
        }
        write!(w, ") ")?;
        if let Type::Primitive(PrimitiveType::Void) = &def.ret {
            // don't print void
        } else {
            write!(w, "-> ")?;
            self.print_ty(&def.ret, w)?;
        }
        self.print_metadatas(def, w)?;
        writeln!(w, "{{")?;
        self.margin += 4;
        self.print_cfg(&def.cfg, &def.vars, w)?;
        self.margin -= 4;
        self.print_margin(w)?;
        writeln!(w, "}}")
    }
    fn print_cont(&mut self, cont: &Contract, w: &mut dyn Write) -> Result {
        writeln!(w, "contract {} {{", cont.name)?;
        self.margin += 4;

        self.print_margin(w)?;
        writeln!(w, "state {{")?;
        self.margin += 4;
        for (name, ty) in cont.states.iter() {
            self.print_margin(w)?;
            write!(w, "{name}: ")?;
            self.print_ty(ty, w)?;
            writeln!(w, ",")?;
        }
        self.margin -= 4;
        self.print_margin(w)?;
        writeln!(w, "}}")?;

        for (_, func_def) in cont.functions.iter() {
            self.print_func_def(func_def, w)?;
            writeln!(w)?;
        }
        self.margin -= 4;
        writeln!(w, "}}")
    }

    fn print_cfg(
        &mut self,
        cfg: &ControlFlowGraph,
        vars: &IndexMap<IdentifierId, Type>,
        w: &mut dyn Write,
    ) -> Result {
        let entry = cfg.entry;

        self.print_basic_block(cfg.basic_blocks.get(&entry).unwrap(), vars, w)?;
        for (id, bb) in cfg.basic_blocks.iter() {
            if *id != entry {
                self.print_basic_block(bb, vars, w)?;
            }
        }
        Ok(())
    }

    fn print_basic_block(
        &mut self,
        bb: &BasicBlock,
        vars: &IndexMap<IdentifierId, Type>,
        w: &mut dyn Write,
    ) -> Result {
        self.print_margin(w)?;
        self.margin += 4;
        writeln!(w, "{}:", bb.id)?;
        for instr in bb.instrs.iter() {
            self.print_margin(w)?;
            self.print_instr(instr, vars, w)?;
            writeln!(w)?;
        }
        self.margin -= 4;
        Ok(())
    }

    fn print_instr(
        &mut self,
        instr: &Instr,
        vars: &IndexMap<IdentifierId, Type>,
        w: &mut dyn Write,
    ) -> Result {
        match &instr.inner {
            InstrDescription::Declaration { id, init_val, ty } => {
                write!(w, "let %{id}: ")?;
                self.print_ty(ty, w)?;
                self.print_metadatas(instr, w)?;
                if let Some(ref expr) = init_val {
                    write!(w, "= ")?;
                    self.print_expr(expr, vars, w)?;
                }
                return Ok(());
            }
            InstrDescription::Assignment { id, val } => {
                write!(w, "%{id}")?;
                self.print_metadatas(instr, w)?;
                write!(w, "= ")?;
                self.print_expr(val, vars, w)?;
                return Ok(());
            }
            InstrDescription::Ret { val } => {
                write!(w, "ret(")?;
                if let Some(ref expr) = val {
                    self.print_expr(expr, vars, w)?;
                    write!(w, ", ")?;
                }
                write!(w, ")")?;
            }
            InstrDescription::Br { target } => write!(w, "br(bb {target}, )")?,

            InstrDescription::BrIf {
                cond,
                then_bb,
                else_bb,
            } => {
                write!(w, "br_if(")?;
                self.print_expr(cond, vars, w)?;
                write!(w, ", bb {then_bb}, bb {else_bb}, )")?;
            }
            InstrDescription::Match {
                val,
                otherwise,
                jump_table,
            } => {
                write!(w, "match(")?;
                self.print_expr(val, vars, w)?;
                write!(w, ", bb {otherwise}, ")?;
                for (expect_val, target) in jump_table.iter() {
                    write!(w, "{expect_val}: i32, bb {target}, ")?;
                }
                write!(w, ")")?;
            }
            InstrDescription::Not { op } => {
                write!(w, "not(")?;
                self.print_expr(op, vars, w)?;
                write!(w, ", )")?;
            }
            InstrDescription::BitNot { op } => {
                write!(w, "bit_not(")?;
                self.print_expr(op, vars, w)?;
                write!(w, ", )")?;
            }
            InstrDescription::Binary { op_code, op1, op2 } => {
                write!(w, "{op_code}")?;
                write!(w, "(")?;
                self.print_expr(op1, vars, w)?;
                write!(w, ", ")?;
                self.print_expr(op2, vars, w)?;
                write!(w, ", )")?;
            }
            InstrDescription::Cmp { op_code, op1, op2 } => {
                match op_code {
                    CmpOp::Eq => write!(w, "eq")?,
                    CmpOp::Ne => write!(w, "ne")?,
                    CmpOp::Gt => write!(w, "gt")?,
                    CmpOp::Ge => write!(w, "ge")?,
                    CmpOp::Lt => write!(w, "lt")?,
                    CmpOp::Le => write!(w, "le")?,
                };
                write!(w, "(")?;
                self.print_expr(op1, vars, w)?;
                write!(w, ", ")?;
                self.print_expr(op2, vars, w)?;
                write!(w, ", )")?;
            }
            InstrDescription::Alloca { ty } => {
                write!(w, "alloca(")?;
                self.print_ty(ty, w)?;
                write!(w, ", )")?;
            }
            InstrDescription::Malloc { ty } => {
                write!(w, "malloc(")?;
                self.print_ty(ty, w)?;
                write!(w, ", )")?;
            }
            InstrDescription::Free { ptr } => {
                write!(w, "free(")?;
                self.print_expr(ptr, vars, w)?;
                write!(w, ", )")?;
            }
            InstrDescription::GetField {
                ptr,
                field_path,
                field_ty,
            } => {
                write!(w, "get_field(")?;
                self.print_expr(ptr, vars, w)?;
                write!(w, ", ")?;
                for field_id in field_path.iter() {
                    write!(w, "{field_id}: i32, ")?;
                }
                write!(w, ") -> ")?;
                self.print_ty(field_ty, w)?;
            }
            InstrDescription::SetField {
                ptr,
                val,
                field_path,
            } => {
                write!(w, "set_field(")?;
                self.print_expr(ptr, vars, w)?;
                write!(w, ", ")?;
                self.print_expr(val, vars, w)?;
                write!(w, ", ")?;
                for field_id in field_path.iter() {
                    write!(w, "{field_id}: i32, ")?;
                }
                write!(w, ")")?;
            }
            InstrDescription::GetStoragePath { storage_path } => {
                write!(w, "get_storage_path(")?;
                for expr in storage_path.iter() {
                    self.print_expr(expr, vars, w)?;
                    write!(w, ", ")?;
                }
                write!(w, ")")?;
            }
            InstrDescription::StorageLoad {
                storage_path,
                load_ty,
            } => {
                write!(w, "storage_load(")?;
                self.print_expr(storage_path, vars, w)?;
                write!(w, ", ) -> ")?;
                self.print_ty(load_ty, w)?;
            }
            InstrDescription::StorageStore {
                storage_path,
                store_val,
            } => {
                write!(w, "storage_store(")?;
                self.print_expr(storage_path, vars, w)?;
                write!(w, ", ")?;
                self.print_expr(store_val, vars, w)?;
                write!(w, ", )")?;
            }
            InstrDescription::Call {
                func_name,
                args,
                ret_ty,
            } => {
                write!(w, "call(@{}(", func_name.get_name())?;
                for expr in args.iter() {
                    self.print_expr(expr, vars, w)?;
                    write!(w, ", ")?;
                }
                write!(w, ") -> ")?;
                self.print_ty(ret_ty, w)?;
                write!(w, ", )")?;
            }
            InstrDescription::IntCast { val, target_ty } => {
                write!(w, "int_cast(")?;
                self.print_expr(val, vars, w)?;
                write!(w, ", ) -> ")?;
                self.print_ty(target_ty, w)?;
            }
        };
        self.print_metadatas(instr, w)
    }

    fn print_expr(
        &mut self,
        expr: &Expr,
        vars: &IndexMap<IdentifierId, Type>,
        w: &mut dyn Write,
    ) -> Result {
        match expr {
            Expr::Identifier(id) => {
                write!(w, "%{id}: ")?;
                self.print_ty(vars.get(id).unwrap(), w)
            }
            Expr::Instr(instr) => self.print_instr(instr, vars, w),
            Expr::Literal(lit) => self.print_literal(lit, w),
            Expr::NOP => unreachable!(),
        }
    }

    fn print_literal(&mut self, lit: &Literal, w: &mut dyn Write) -> Result {
        match lit {
            Literal::Str(val) => write!(w, "\"{val}\": str"),
            Literal::Bool(val) => {
                if *val {
                    write!(w, "true")?;
                } else {
                    write!(w, "false")?;
                }
                write!(w, ": bool")
            }
            Literal::Int(int_val) => match int_val {
                IntLiteral::I8(val) => write!(w, "{val}: i8"),
                IntLiteral::I16(val) => write!(w, "{val}: i16"),
                IntLiteral::I32(val) => write!(w, "{val}: i32"),
                IntLiteral::I64(val) => write!(w, "{val}: i64"),
                IntLiteral::I128(val) => write!(w, "{val}: i128"),
                IntLiteral::I256(val) => write!(w, "{val}: i256"),
                IntLiteral::U8(val) => write!(w, "{val}: u8"),
                IntLiteral::U16(val) => write!(w, "{val}: u16"),
                IntLiteral::U32(val) => write!(w, "{val}: u32"),
                IntLiteral::U64(val) => write!(w, "{val}: u64"),
                IntLiteral::U128(val) => write!(w, "{val}: u128"),
                IntLiteral::U256(val) => write!(w, "{val}: u256"),
            },
        }
    }
}
