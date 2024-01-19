// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use crate::ir::{
    cfg::parse_builtin_type,
    context::IRContext,
    interface_type::{parse_intrinsic_func_name, PartialFuncName},
};
use indexmap::IndexMap;

use crate::ir::{
    builder::{BasicBlockId, IdentifierId, MetaDataId},
    cfg, frontend,
};

pub fn translate_main_module(ctx: &mut IRContext, frontend_module: &frontend::Module) {
    ctx.main_module = frontend_module.name.clone();
    translate_module(ctx, frontend_module);
}

pub fn translate_module(ctx: &mut IRContext, frontend_module: &frontend::Module) {
    let module_name = frontend_module.name.clone();
    ctx.main_module = module_name.clone();
    let mut cfg_module = cfg::Module {
        name: module_name.clone(),
        types: IndexMap::default(),
        functions: IndexMap::default(),
        contract: None,
    };

    translate_defs(ctx, &mut cfg_module, &frontend_module.defs);
    ctx.modules.borrow_mut().insert(module_name, cfg_module);
}

fn translate_defs(ctx: &mut IRContext, module: &mut cfg::Module, defs: &frontend::Defs) {
    let mut cur_defs = defs;
    loop {
        match cur_defs {
            frontend::Defs::None => break,
            frontend::Defs::Some(def, next) => {
                cur_defs = next;
                translate_def(ctx, module, def);
            }
        }
    }
}

fn translate_def(ctx: &mut IRContext, module: &mut cfg::Module, def: &frontend::Def) {
    match def {
        frontend::Def::TypeDef(def) => translate_type_def(module, def),
        frontend::Def::FuncDef(def) => translate_func_def(module, def),
        frontend::Def::ContDef(def) => translate_cont_def(module, def),
        frontend::Def::MDDef(def) => translate_md_def(ctx, def),
    }
}

fn translate_md_def(ctx: &mut IRContext, def: &frontend::MDDef) {
    let mut md = cfg::MetaData::default();

    let mut cur_fields = def.data.as_ref();
    loop {
        match cur_fields {
            frontend::MDFields::None => break,
            frontend::MDFields::Some(field, next) => {
                cur_fields = next;
                md.push_field(match field.as_ref() {
                    frontend::MDField::Ident(_) => unimplemented!(),
                    frontend::MDField::Bool(val) => cfg::Literal::Bool(*val),
                    frontend::MDField::Int(int_val) => {
                        cfg::Literal::Int(translate_int_value(int_val))
                    }
                    frontend::MDField::Str(val) => cfg::Literal::Str(val.clone()),
                });
            }
        }
    }
    ctx.metadata.borrow_mut().insert(def.id as MetaDataId, md);
}

fn translate_type_def(module: &mut cfg::Module, def: &frontend::TypeDef) {
    let ty_def = match def {
        frontend::TypeDef::Compound(name, fields, metadatas) => {
            let mut cfg_fields = Vec::default();
            let mut cur_fields = fields.as_ref();
            loop {
                match cur_fields {
                    frontend::Fields::None => break,
                    frontend::Fields::Some(field, next) => {
                        cur_fields = next;
                        cfg_fields.push(cfg::Field {
                            name: field.name.clone(),
                            ty: Rc::new(translate_type(module, &field.ty)),
                        });
                    }
                }
            }

            cfg::TypeDefinition {
                name: name.clone(),
                kind: map_cfg_ty_kind(name),
                ty: Rc::new(cfg::Type::Compound(Rc::new(cfg_fields))),
                metadata: translate_metadatas(metadatas),
            }
        }
        frontend::TypeDef::Alias(name, ty, metadatas) => cfg::TypeDefinition {
            name: name.clone(),
            kind: cfg::TypeDefinitionKind::Alias,
            ty: Rc::new(translate_type(module, ty)),
            metadata: translate_metadatas(metadatas),
        },
    };
    module.types.insert(ty_def.name.clone(), Rc::new(ty_def));
}

fn map_cfg_ty_kind(name: &str) -> cfg::TypeDefinitionKind {
    let paths: Vec<&str> = name.split('.').collect();
    let prefix = paths
        .first()
        .unwrap_or_else(|| panic!("illegal type name : {name}"));
    match *prefix {
        "struct" => cfg::TypeDefinitionKind::Struct,
        "ir" => cfg::TypeDefinitionKind::Builtin,
        "enum" => cfg::TypeDefinitionKind::Enum,
        _ => cfg::TypeDefinitionKind::Alias,
    }
}

fn translate_func_def(module: &mut cfg::Module, def: &frontend::FuncDef) {
    let mut func = cfg::FunctionDefinition {
        name: def.name.clone(),
        params: translate_params(module, &def.params),
        ret: translate_type(module, &def.ret),
        is_external: def.vis == frontend::Visibility::Public,
        cfg: cfg::ControlFlowGraph::default(),
        vars: IndexMap::default(),
        metadata: translate_metadatas(&def.metadatas),
    };
    for (idx, param_ty) in func.params.iter().enumerate() {
        func.vars.insert(idx as IdentifierId, param_ty.clone());
    }
    translate_basic_blocks(module, &mut func, &def.basic_blocks);
    module.functions.insert(func.name.clone(), func);
}

fn translate_cont_def(module: &mut cfg::Module, def: &frontend::ContDef) {
    if module.contract.is_some() {
        panic!("a single module can only define one contract ")
    }
    let mut cont = cfg::Contract {
        name: def.name.clone(),
        ..Default::default()
    };

    let mut cur_funcs = def.funcs.as_ref();

    translate_states(module, &mut cont, &def.states);

    loop {
        match cur_funcs {
            frontend::Defs::None => break,
            frontend::Defs::Some(def, next) => {
                if let frontend::Def::FuncDef(def) = def.as_ref() {
                    cur_funcs = next;
                    let mut func: cfg::FunctionDefinition = cfg::FunctionDefinition {
                        name: def.name.clone(),
                        params: translate_params(module, &def.params),
                        ret: translate_type(module, &def.ret),
                        is_external: def.vis == frontend::Visibility::Public,
                        cfg: cfg::ControlFlowGraph::default(),
                        vars: IndexMap::default(),
                        metadata: translate_metadatas(&def.metadatas),
                    };
                    for (idx, param_ty) in func.params.iter().enumerate() {
                        func.vars.insert(idx as IdentifierId, param_ty.clone());
                    }
                    translate_basic_blocks(module, &mut func, &def.basic_blocks);
                    cont.functions.insert(func.name.clone(), func);
                } else {
                    panic!("only functions can be defined within the contract")
                }
            }
        }
    }

    module.contract = Some(cont);
}

fn translate_states(module: &mut cfg::Module, cont: &mut cfg::Contract, states: &frontend::Fields) {
    let mut cur_states = states;
    loop {
        match cur_states {
            frontend::Fields::None => break,
            frontend::Fields::Some(state, next) => {
                cur_states = next;
                cont.states
                    .insert(state.name.clone(), translate_type(module, &state.ty));
            }
        }
    }
}

fn translate_params(module: &mut cfg::Module, params: &frontend::Params) -> Vec<cfg::Type> {
    let mut cfg_params = Vec::default();
    let mut cur_params = params;
    loop {
        match cur_params {
            frontend::Params::None => break,
            frontend::Params::Some(param, next) => {
                cur_params = next;
                cfg_params.push(translate_type(module, &param.ty));
            }
        }
    }
    cfg_params
}

fn translate_basic_blocks(
    module: &mut cfg::Module,
    func: &mut cfg::FunctionDefinition,
    bbs: &frontend::BasicBlocks,
) {
    let mut cur_bbs;
    match bbs {
        frontend::BasicBlocks::None => return,
        frontend::BasicBlocks::Some(bb, next) => {
            cur_bbs = next.as_ref();
            func.cfg.entry = bb.id as BasicBlockId;
            translate_basic_block(module, func, bb);
        }
    }
    loop {
        match cur_bbs {
            frontend::BasicBlocks::None => break,
            frontend::BasicBlocks::Some(bb, next) => {
                cur_bbs = next;
                translate_basic_block(module, func, bb);
            }
        }
    }
}

fn translate_basic_block(
    module: &mut cfg::Module,
    func: &mut cfg::FunctionDefinition,
    bb: &frontend::BasicBlock,
) {
    let mut cfg_bb = cfg::BasicBlock {
        id: bb.id as u32,
        instrs: Vec::default(),
    };
    translate_instrs(module, func, &mut cfg_bb, &bb.instrs);
    func.cfg.basic_blocks.insert(cfg_bb.id, cfg_bb);
}

fn translate_instrs(
    module: &mut cfg::Module,
    func: &mut cfg::FunctionDefinition,
    bb: &mut cfg::BasicBlock,
    instrs: &frontend::Instrs,
) {
    let mut cur_instrs = instrs;
    loop {
        match cur_instrs {
            frontend::Instrs::None => break,
            frontend::Instrs::Some(instr, next) => {
                cur_instrs = next;
                bb.insert_instr(translate_instr(module, func, instr));
            }
        }
    }
}

fn translate_instr(
    module: &mut cfg::Module,
    func: &mut cfg::FunctionDefinition,
    instr: &frontend::Instr,
) -> cfg::Instr {
    match instr {
        frontend::Instr::Simple(instr) => translate_simple_instr(module, instr),
        frontend::Instr::Declaration(id, init, ty, metadatas) => {
            let cfg_ty = translate_type(module, ty);
            func.vars.insert(*id as IdentifierId, cfg_ty.clone());
            cfg::Instr {
                inner: cfg::InstrDescription::declaration(
                    *id as IdentifierId,
                    init.as_ref().map(|expr| translate_expr(module, expr)),
                    cfg_ty,
                ),
                metadata: translate_metadatas(metadatas),
            }
        }
        frontend::Instr::Assignment(id, val, metadatas) => cfg::Instr {
            inner: cfg::InstrDescription::assignment(
                *id as IdentifierId,
                translate_expr(module, val),
            ),
            metadata: translate_metadatas(metadatas),
        },
    }
}

fn translate_expr(module: &mut cfg::Module, expr: &frontend::Expr) -> cfg::Expr {
    match expr {
        frontend::Expr::Instr(instr) => {
            cfg::Expr::Instr(Box::new(translate_simple_instr(module, instr)))
        }
        frontend::Expr::Ident(id, _) => cfg::Expr::Identifier(*id as IdentifierId),
        frontend::Expr::Bool(val) => cfg::Expr::Literal(cfg::Literal::Bool(*val)),
        frontend::Expr::Int(int_val) => {
            cfg::Expr::Literal(cfg::Literal::Int(translate_int_value(int_val)))
        }
        frontend::Expr::Str(val) => cfg::Expr::Literal(cfg::Literal::Str(val.clone())),
        _ => panic!("untranslatable expression: {expr:?}"),
    }
}

fn translate_int_value(int_val: &frontend::IntValue) -> cfg::IntLiteral {
    if int_val.positive {
        match int_val.ty.as_ref() {
            frontend::Type::Int(int_ty) => match int_ty {
                frontend::IntType::I8 => cfg::IntLiteral::I8(int_val.val as i8),
                frontend::IntType::I16 => cfg::IntLiteral::I16(int_val.val as i16),
                frontend::IntType::I32 => cfg::IntLiteral::I32(int_val.val as i32),
                frontend::IntType::I64 => cfg::IntLiteral::I64(int_val.val as i64),
                frontend::IntType::I128 => cfg::IntLiteral::I128(int_val.val as i128),
                frontend::IntType::U8 => cfg::IntLiteral::U8(int_val.val as u8),
                frontend::IntType::U16 => cfg::IntLiteral::U16(int_val.val as u16),
                frontend::IntType::U32 => cfg::IntLiteral::U32(int_val.val as u32),
                frontend::IntType::U64 => cfg::IntLiteral::U64(int_val.val as u64),
                frontend::IntType::U128 => cfg::IntLiteral::U128(int_val.val),
            },
            _ => panic!("illegal int literal: {int_val:?}"),
        }
    } else {
        match int_val.ty.as_ref() {
            frontend::Type::Int(int_ty) => match int_ty {
                frontend::IntType::I8 => cfg::IntLiteral::I8(-(int_val.val as i8)),
                frontend::IntType::I16 => cfg::IntLiteral::I16(-(int_val.val as i16)),
                frontend::IntType::I32 => cfg::IntLiteral::I32(-(int_val.val as i32)),
                frontend::IntType::I64 => cfg::IntLiteral::I64(-(int_val.val as i64)),
                frontend::IntType::I128 => cfg::IntLiteral::I128(-(int_val.val as i128)),
                _ => panic!("illegal negetive int literal: {int_val:?}"),
            },
            _ => panic!("illegal int literal: {int_val:?}"),
        }
    }
}

fn translate_simple_instr(module: &mut cfg::Module, instr: &frontend::SimpleInstr) -> cfg::Instr {
    match instr.name.as_str() {
        "call" => translate_call_instr(module, instr),
        "ret" => translate_ret_instr(module, instr),
        "br" => translate_br_instr(instr),
        "br_if" => translate_br_if_instr(module, instr),
        "match" => translate_match_instr(module, instr),
        "not" => translate_not_instr(module, instr),
        "bit_not" => translate_bit_not_instr(module, instr),
        "add" | "sub" | "mul" | "div" | "mod" | "exp" | "and" | "bit_and" | "or" | "bit_or"
        | "bit_xor" | "shl" | "shr" | "sar" => translate_binary_instr(module, instr),
        "eq" | "ne" | "gt" | "ge" | "lt" | "le" => translate_cmp_instr(module, instr),
        "alloca" => translate_alloca_instr(module, instr),
        "malloc" => translate_malloc_instr(module, instr),
        "free" => translate_free_instr(module, instr),
        "get_field" => translate_get_field_instr(module, instr),
        "set_field" => translate_set_field_instr(module, instr),
        "get_storage_path" => translate_get_storage_path_instr(module, instr),
        "storage_load" => translate_storage_load_instr(module, instr),
        "storage_store" => translate_storage_store_instr(module, instr),
        "int_cast" => translate_int_cast_instr(module, instr),
        _ => panic!("unknown instruction: {}", instr.name),
    }
}

fn translate_int_cast_instr(module: &mut cfg::Module, instr: &frontend::SimpleInstr) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal int_cast instr: {instr:?}"),
        frontend::Exprs::Some(expr, next) => {
            if let frontend::Exprs::None = next.as_ref() {
                cfg::Instr {
                    inner: cfg::InstrDescription::int_cast(
                        translate_expr(module, expr),
                        translate_type(module, &instr.ret),
                    ),
                    metadata: translate_metadatas(&instr.metadatas),
                }
            } else {
                panic!("illegal storage_load instr: {instr:?}");
            }
        }
    }
}

fn translate_storage_store_instr(
    module: &mut cfg::Module,
    instr: &frontend::SimpleInstr,
) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal storage_load instr: {instr:?}"),
        frontend::Exprs::Some(path, next) => match next.as_ref() {
            frontend::Exprs::None => panic!("illegal storage_load instr: {instr:?}"),
            frontend::Exprs::Some(val, next) => {
                if let frontend::Exprs::None = next.as_ref() {
                    cfg::Instr {
                        inner: cfg::InstrDescription::storage_store(
                            translate_expr(module, path),
                            translate_expr(module, val),
                        ),
                        metadata: translate_metadatas(&instr.metadatas),
                    }
                } else {
                    panic!("illegal storage_load instr: {instr:?}");
                }
            }
        },
    }
}

fn translate_storage_load_instr(
    module: &mut cfg::Module,
    instr: &frontend::SimpleInstr,
) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal storage_load instr: {instr:?}"),
        frontend::Exprs::Some(val, next) => {
            if let frontend::Exprs::None = next.as_ref() {
                cfg::Instr {
                    inner: cfg::InstrDescription::storage_load(
                        translate_expr(module, val),
                        translate_type(module, &instr.ret),
                    ),
                    metadata: translate_metadatas(&instr.metadatas),
                }
            } else {
                panic!("illegal storage_load instr: {instr:?}");
            }
        }
    }
}

fn translate_get_storage_path_instr(
    module: &mut cfg::Module,
    instr: &frontend::SimpleInstr,
) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal get_storage_path instr: {instr:?}"),
        frontend::Exprs::Some(val, next) => {
            let mut storage_path = Vec::default();
            storage_path.push(translate_expr(module, val));
            let mut cur_exprs = next.as_ref();
            loop {
                match cur_exprs {
                    frontend::Exprs::None => break,
                    frontend::Exprs::Some(val, next) => {
                        cur_exprs = next;
                        storage_path.push(translate_expr(module, val));
                    }
                }
            }
            cfg::Instr {
                inner: cfg::InstrDescription::get_storage_path(storage_path),
                metadata: translate_metadatas(&instr.metadatas),
            }
        }
    }
}

fn translate_set_field_instr(
    module: &mut cfg::Module,
    instr: &frontend::SimpleInstr,
) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal set_field instr: {instr:?}"),
        frontend::Exprs::Some(ptr, next) => match next.as_ref() {
            frontend::Exprs::None => panic!("illegal set_field instr: {instr:?}"),
            frontend::Exprs::Some(val, next) => {
                let mut field_path = Vec::<u32>::default();
                let mut cur_exprs = next.as_ref();
                loop {
                    match cur_exprs {
                        frontend::Exprs::None => break,
                        frontend::Exprs::Some(val, next) => {
                            cur_exprs = next;
                            if let frontend::Expr::Int(int_val) = val.as_ref() {
                                field_path.push(int_val.val as u32);
                            } else {
                                panic!("illegal set_field instr: {instr:?}");
                            }
                        }
                    }
                }
                cfg::Instr {
                    inner: cfg::InstrDescription::set_field(
                        translate_expr(module, ptr),
                        field_path,
                        translate_expr(module, val),
                    ),
                    metadata: translate_metadatas(&instr.metadatas),
                }
            }
        },
    }
}

fn translate_get_field_instr(
    module: &mut cfg::Module,
    instr: &frontend::SimpleInstr,
) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal get_field instr: {instr:?}"),
        frontend::Exprs::Some(ptr, next) => {
            let mut field_path = Vec::<u32>::default();
            let mut cur_exprs = next.as_ref();
            loop {
                match cur_exprs {
                    frontend::Exprs::None => break,
                    frontend::Exprs::Some(val, next) => {
                        cur_exprs = next;
                        if let frontend::Expr::Int(int_val) = val.as_ref() {
                            field_path.push(int_val.val as u32);
                        } else {
                            panic!("illegal get_field instr: {instr:?}");
                        }
                    }
                }
            }
            cfg::Instr {
                inner: cfg::InstrDescription::get_field(
                    translate_expr(module, ptr),
                    field_path,
                    translate_type(module, &instr.ret),
                ),
                metadata: translate_metadatas(&instr.metadatas),
            }
        }
    }
}

fn translate_free_instr(module: &mut cfg::Module, instr: &frontend::SimpleInstr) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal free instr: {instr:?}"),
        frontend::Exprs::Some(expr, next) => {
            if let frontend::Exprs::None = next.as_ref() {
                cfg::Instr {
                    inner: cfg::InstrDescription::free(translate_expr(module, expr)),
                    metadata: translate_metadatas(instr.metadatas.as_ref()),
                }
            } else {
                panic!("illegal free instr: {instr:?}")
            }
        }
    }
}

fn translate_malloc_instr(module: &mut cfg::Module, instr: &frontend::SimpleInstr) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal malloc instr: {instr:?}"),
        frontend::Exprs::Some(expr, next) => {
            if let (frontend::Expr::Type(ty), frontend::Exprs::None) =
                (expr.as_ref(), next.as_ref())
            {
                cfg::Instr {
                    inner: cfg::InstrDescription::malloc(translate_type(module, ty)),
                    metadata: translate_metadatas(instr.metadatas.as_ref()),
                }
            } else {
                panic!("illegal malloc instr: {instr:?}")
            }
        }
    }
}

fn translate_alloca_instr(module: &mut cfg::Module, instr: &frontend::SimpleInstr) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal alloca instr: {instr:?}"),
        frontend::Exprs::Some(expr, next) => {
            if let (frontend::Expr::Type(ty), frontend::Exprs::None) =
                (expr.as_ref(), next.as_ref())
            {
                cfg::Instr {
                    inner: cfg::InstrDescription::alloca(translate_type(module, ty)),
                    metadata: translate_metadatas(instr.metadatas.as_ref()),
                }
            } else {
                panic!("illegal alloca instr: {instr:?}")
            }
        }
    }
}

fn translate_cmp_instr(module: &mut cfg::Module, instr: &frontend::SimpleInstr) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal cmp instr: {instr:?}"),
        frontend::Exprs::Some(op1, next) => match next.as_ref() {
            frontend::Exprs::None => panic!("illegal cmp instr: {instr:?}"),
            frontend::Exprs::Some(op2, next) => {
                if let frontend::Exprs::None = next.as_ref() {
                    cfg::Instr {
                        inner: cfg::InstrDescription::Cmp {
                            op_code: map_cmp_op(&instr.name),
                            op1: translate_expr(module, op1),
                            op2: translate_expr(module, op2),
                        },
                        metadata: translate_metadatas(&instr.metadatas),
                    }
                } else {
                    panic!("illegal cmp instr: {instr:?}")
                }
            }
        },
    }
}

fn map_cmp_op(name: &str) -> cfg::CmpOp {
    match name {
        "eq" => cfg::CmpOp::Eq,
        "ne" => cfg::CmpOp::Ne,
        "gt" => cfg::CmpOp::Gt,
        "ge" => cfg::CmpOp::Ge,
        "lt" => cfg::CmpOp::Lt,
        "le" => cfg::CmpOp::Le,
        _ => panic!("unknown cmp op: {name}"),
    }
}

fn translate_binary_instr(module: &mut cfg::Module, instr: &frontend::SimpleInstr) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal binary instr: {instr:?}"),
        frontend::Exprs::Some(op1, next) => match next.as_ref() {
            frontend::Exprs::None => panic!("illegal binary instr: {instr:?}"),
            frontend::Exprs::Some(op2, next) => {
                if let frontend::Exprs::None = next.as_ref() {
                    cfg::Instr {
                        inner: cfg::InstrDescription::Binary {
                            op_code: map_binary_op(&instr.name),
                            op1: translate_expr(module, op1),
                            op2: translate_expr(module, op2),
                        },
                        metadata: translate_metadatas(&instr.metadatas),
                    }
                } else {
                    panic!("illegal binary instr: {instr:?}")
                }
            }
        },
    }
}

fn map_binary_op(name: &str) -> cfg::BinaryOp {
    match name {
        "add" => cfg::BinaryOp::Add,
        "sub" => cfg::BinaryOp::Sub,
        "mul" => cfg::BinaryOp::Mul,
        "div" => cfg::BinaryOp::Div,
        "mod" => cfg::BinaryOp::Mod,
        "exp" => cfg::BinaryOp::Exp,
        "and" => cfg::BinaryOp::And,
        "bit_and" => cfg::BinaryOp::BitAnd,
        "or" => cfg::BinaryOp::Or,
        "bit_or" => cfg::BinaryOp::BitOr,
        "bit_xor" => cfg::BinaryOp::BitXor,
        _ => panic!("unknown binary op: {name}"),
    }
}
fn translate_call_instr(module: &mut cfg::Module, instr: &frontend::SimpleInstr) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal call instr: {instr:?}"),
        frontend::Exprs::Some(invoke, next) => {
            if let (frontend::Expr::Invoke(name, args, ret), frontend::Exprs::None) =
                (invoke.as_ref(), next.as_ref())
            {
                let mut cur_args = args.as_ref();
                let mut cfg_args = Vec::default();
                loop {
                    match cur_args {
                        frontend::Exprs::None => break,
                        frontend::Exprs::Some(arg, next) => {
                            cur_args = next;
                            cfg_args.push(translate_expr(module, arg));
                        }
                    }
                }
                let func_name = match parse_intrinsic_func_name(name) {
                    Some(intrinsic) => PartialFuncName::from(intrinsic),
                    None => PartialFuncName::from(name.clone()),
                };
                cfg::Instr {
                    inner: cfg::InstrDescription::call(
                        func_name,
                        cfg_args,
                        translate_type(module, &ret),
                    ),
                    metadata: translate_metadatas(instr.metadatas.as_ref()),
                }
            } else {
                panic!("illegal call instr: {instr:?}")
            }
        }
    }
}

fn translate_not_instr(module: &mut cfg::Module, instr: &frontend::SimpleInstr) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal not instr: {instr:?}"),
        frontend::Exprs::Some(op, next) => {
            if let frontend::Exprs::None = next.as_ref() {
                cfg::Instr {
                    inner: cfg::InstrDescription::not(translate_expr(module, op)),
                    metadata: translate_metadatas(&instr.metadatas),
                }
            } else {
                panic!("illegal not instr: {instr:?}")
            }
        }
    }
}

fn translate_bit_not_instr(module: &mut cfg::Module, instr: &frontend::SimpleInstr) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal bit_not instr: {instr:?}"),
        frontend::Exprs::Some(op, next) => {
            if let frontend::Exprs::None = next.as_ref() {
                cfg::Instr {
                    inner: cfg::InstrDescription::bit_not(translate_expr(module, op)),
                    metadata: translate_metadatas(&instr.metadatas),
                }
            } else {
                panic!("illegal bit_not instr: {instr:?}")
            }
        }
    }
}

fn translate_ret_instr(module: &mut cfg::Module, instr: &frontend::SimpleInstr) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => cfg::Instr {
            inner: cfg::InstrDescription::ret(None),
            metadata: translate_metadatas(instr.metadatas.as_ref()),
        },
        frontend::Exprs::Some(expr, next) => {
            if let frontend::Exprs::None = next.as_ref() {
                cfg::Instr {
                    inner: cfg::InstrDescription::ret(Some(translate_expr(module, expr))),
                    metadata: translate_metadatas(instr.metadatas.as_ref()),
                }
            } else {
                panic!("illegal ret instr: {instr:?}");
            }
        }
    }
}

fn translate_br_instr(instr: &frontend::SimpleInstr) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal br instr: {instr:?}"),
        frontend::Exprs::Some(label, next) => {
            if let (frontend::Expr::Label(id), frontend::Exprs::None) =
                (label.as_ref(), next.as_ref())
            {
                cfg::Instr {
                    inner: cfg::InstrDescription::br(*id as BasicBlockId),
                    metadata: translate_metadatas(instr.metadatas.as_ref()),
                }
            } else {
                panic!("illegal br instr: {instr:?}")
            }
        }
    }
}

fn translate_br_if_instr(module: &mut cfg::Module, instr: &frontend::SimpleInstr) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal br_if instr: {instr:?}"),
        frontend::Exprs::Some(cond, next) => match next.as_ref() {
            frontend::Exprs::None => panic!("illegal br_if instr: {instr:?}"),
            frontend::Exprs::Some(then_bb, next) => {
                if let frontend::Expr::Label(then_id) = then_bb.as_ref() {
                    match next.as_ref() {
                        frontend::Exprs::None => panic!("illegal br_if instr: {instr:?}"),
                        frontend::Exprs::Some(else_bb, next) => {
                            if let (frontend::Expr::Label(else_id), frontend::Exprs::None) =
                                (else_bb.as_ref(), next.as_ref())
                            {
                                cfg::Instr {
                                    inner: cfg::InstrDescription::br_if(
                                        translate_expr(module, cond),
                                        *then_id as BasicBlockId,
                                        *else_id as BasicBlockId,
                                    ),
                                    metadata: translate_metadatas(instr.metadatas.as_ref()),
                                }
                            } else {
                                panic!("illegal br_if instr: {instr:?}")
                            }
                        }
                    }
                } else {
                    panic!("illegal br_if instr: {instr:?}")
                }
            }
        },
    }
}

fn translate_match_instr(module: &mut cfg::Module, instr: &frontend::SimpleInstr) -> cfg::Instr {
    match instr.args.as_ref() {
        frontend::Exprs::None => panic!("illegal match instr: {instr:?}"),
        frontend::Exprs::Some(val, next) => match next.as_ref() {
            frontend::Exprs::None => panic!("illegal match instr: {instr:?}"),
            frontend::Exprs::Some(default_bb, next) => {
                if let frontend::Expr::Label(default_id) = default_bb.as_ref() {
                    let mut jump_table = IndexMap::<u32, BasicBlockId>::default();
                    let mut cur_exprs = next.as_ref();
                    loop {
                        match cur_exprs {
                            frontend::Exprs::None => break,
                            frontend::Exprs::Some(val, next) => {
                                if let frontend::Expr::Int(int_val) = val.as_ref() {
                                    match next.as_ref() {
                                        frontend::Exprs::None => {
                                            panic!("illegal match instr: {instr:?}")
                                        }
                                        frontend::Exprs::Some(target_bb, next) => {
                                            if let frontend::Expr::Label(target_id) =
                                                target_bb.as_ref()
                                            {
                                                cur_exprs = next;
                                                jump_table.insert(
                                                    int_val.val as u32,
                                                    *target_id as BasicBlockId,
                                                );
                                            } else {
                                                panic!("illegal match instr: {instr:?}");
                                            }
                                        }
                                    }
                                } else {
                                    panic!("illegal match instr: {instr:?}");
                                }
                            }
                        }
                    }
                    cfg::Instr {
                        inner: cfg::InstrDescription::r#match(
                            translate_expr(module, val),
                            *default_id as BasicBlockId,
                            jump_table,
                        ),
                        metadata: translate_metadatas(&instr.metadatas),
                    }
                } else {
                    panic!("illegal match instr: {instr:?}")
                }
            }
        },
    }
}

fn translate_type(module: &mut cfg::Module, ty: &frontend::Type) -> cfg::Type {
    match ty {
        frontend::Type::Int(int_ty) => {
            cfg::Type::Primitive(cfg::PrimitiveType::Int(match int_ty {
                frontend::IntType::I8 => cfg::IntType::I8,
                frontend::IntType::I16 => cfg::IntType::I16,
                frontend::IntType::I32 => cfg::IntType::I32,
                frontend::IntType::I64 => cfg::IntType::I64,
                frontend::IntType::I128 => cfg::IntType::I128,
                frontend::IntType::U8 => cfg::IntType::U8,
                frontend::IntType::U16 => cfg::IntType::U16,
                frontend::IntType::U32 => cfg::IntType::U32,
                frontend::IntType::U64 => cfg::IntType::U64,
                frontend::IntType::U128 => cfg::IntType::U128,
            }))
        }
        frontend::Type::Bool => cfg::Type::Primitive(cfg::PrimitiveType::Bool),
        frontend::Type::Str => cfg::Type::Primitive(cfg::PrimitiveType::Str),
        frontend::Type::Void => cfg::Type::Primitive(cfg::PrimitiveType::Void),
        frontend::Type::Map(key, val) => cfg::Type::Map {
            key: Rc::new(translate_type(module, key)),
            value: Rc::new(translate_type(module, val)),
        },
        frontend::Type::Array(elem, len) => cfg::Type::Array {
            elem: Rc::new(translate_type(module, elem)),
            len: len.map(|x| x as u32),
        },
        frontend::Type::Pointer(elem) => cfg::Type::Pointer(Rc::new(translate_type(module, elem))),
        frontend::Type::Named(name) => match parse_builtin_type(name) {
            Some(built_ty) => cfg::Type::Builtin(built_ty),
            None => cfg::Type::Def(module.types.get(name).unwrap().clone()),
        },
    }
}

fn translate_metadatas(metadatas: &frontend::MetaDatas) -> IndexMap<String, MetaDataId> {
    let mut cfg_metadatas = IndexMap::<String, MetaDataId>::default();
    let mut cur_metadatas = metadatas;
    loop {
        match cur_metadatas {
            frontend::MetaDatas::None => break,
            frontend::MetaDatas::Some(metadata, next) => {
                cur_metadatas = next;
                cfg_metadatas.insert(metadata.name.clone(), metadata.id as MetaDataId);
            }
        }
    }
    cfg_metadatas
}
