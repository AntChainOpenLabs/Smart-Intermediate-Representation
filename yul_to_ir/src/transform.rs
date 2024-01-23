// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        self, Block, Expression, FunctionCall, FunctionDefinition, Identifier, IfElse,
        InnerSegment, Object, Statement, Switch, TypeName,
    },
    context::{mem_data_ty, Yul2IRContext, WORD_SIZE, WORD_TY},
    instruction::parse_intrinsic_func_name,
};
use indexmap::IndexMap;
use num_bigint::BigInt;
use smart_ir::ir::{
    builder::{IdentifierId, Label},
    cfg::{self, Contract, Expr, Literal, Module, Type},
    context::{ASTLoweringError, CompileResult},
    interface_type::{get_intrinsic_func_by_key, IntrinsicFuncName},
    intrinsic_func::initialize_intrinisic_func_names,
};

#[allow(unused)]
impl Yul2IRContext {
    pub(crate) fn walk_block(&self, block: &crate::ast::Block) -> CompileResult {
        for stmt in &block.statements {
            self.walk_stmt(&stmt);
        }
        self.ok_result()
    }

    pub(crate) fn walk_stmt(&self, stmt: &crate::ast::Statement) -> CompileResult {
        match stmt {
            crate::ast::Statement::Assignment(assign) => self.walk_assignment(assign),
            crate::ast::Statement::VariableDeclaration(var_decl) => {
                self.walk_variable_declaration(&var_decl)
            }
            crate::ast::Statement::If(r#if) => self.walk_if(r#if),
            crate::ast::Statement::For(r#for) => self.walk_for(r#for),
            crate::ast::Statement::Switch(switch) => self.walk_switch(switch),
            crate::ast::Statement::Leave => self.walk_leave(),
            crate::ast::Statement::Break => self.walk_break(),
            crate::ast::Statement::Continue => self.walk_continue(),
            crate::ast::Statement::Block(block) => self.walk_block(block),
            crate::ast::Statement::FunctionDefinition(func_def) => {
                self.walk_function_definition(func_def)
            }
            crate::ast::Statement::FunctionCall(func_call) => {
                let val = self.walk_function_call(func_call).unwrap();
                if let smart_ir::ir::cfg::Expr::Instr(instr) = val {
                    self.ir_context.builder.insert_instr(*instr);
                }
                self.ok_result()
            }
            Statement::IfElse(ifelse) => self.walk_ifelse(ifelse),
        }
    }

    pub(crate) fn walk_function_definition(
        &self,
        func_def: &crate::ast::FunctionDefinition,
    ) -> CompileResult {
        self.ok_result()
    }

    fn walk_ifelse(&self, if_stmt: &crate::ast::IfElse) -> CompileResult {
        let cond_value = self.walk_expr(&if_stmt.cond)?;
        let then_block = self.ir_context.append_block("if_body");
        let end_block = self.ir_context.append_block("if_exit");
        if if_stmt.else_body.statements.is_empty() {
            self.ir_context
                .builder
                .build_cond_br(cond_value, &then_block, &end_block);
            self.ir_context.builder.position_at_end(&then_block);
            self.walk_block(&if_stmt.body)?;
            self.ir_context.builder.build_br(&end_block);
        } else {
            let else_block = self.ir_context.append_block("else_body");
            self.ir_context
                .builder
                .build_cond_br(cond_value, &then_block, &else_block);
            self.ir_context.builder.position_at_end(&then_block);
            self.walk_block(&if_stmt.body);
            self.ir_context.builder.build_br(&end_block);

            self.ir_context.builder.position_at_end(&else_block);
            self.walk_block(&if_stmt.else_body);
            self.ir_context.builder.build_br(&end_block);
        }

        self.ir_context.builder.position_at_end(&end_block);

        self.ok_result()
    }

    fn walk_switch(&self, switch: &crate::ast::Switch) -> CompileResult {
        let if_stmt = switch2ifelse(switch);
        self.walk_ifelse(&if_stmt)
    }

    fn walk_for(&self, r#for: &crate::ast::For) -> CompileResult {
        self.walk_block(&r#for.init_block);
        let cond_block = self.ir_context.append_block("for_cond");
        let body_block = self.ir_context.append_block("for_body");
        let end_block = self.ir_context.append_block("for_exit");
        let label_target: Label = Label::new(&end_block, &cond_block);
        self.push_label_target(label_target);
        self.ir_context.builder.build_br(&cond_block);

        self.ir_context.builder.position_at_end(&cond_block);
        let cond_value = self.walk_expr(&r#for.condition)?;
        self.ir_context
            .builder
            .build_cond_br(cond_value, &body_block, &end_block);

        self.ir_context.builder.position_at_end(&body_block);
        self.walk_block(&r#for.execution_block);
        self.walk_block(&r#for.post_block);
        self.ir_context.builder.build_br(&cond_block);

        self.ir_context.builder.position_at_end(&end_block);
        self.pop_label_target();

        self.ok_result()
    }

    fn walk_if(&self, r#if: &crate::ast::If) -> CompileResult {
        let cond_value = self.walk_expr(&r#if.cond)?;
        let then_block = self.ir_context.append_block("if_body");
        let end_block = self.ir_context.append_block("if_exit");
        self.ir_context
            .builder
            .build_cond_br(cond_value, &then_block, &end_block);

        self.ir_context.builder.position_at_end(&then_block);
        for stmt in &r#if.body.statements {
            self.walk_stmt(stmt)?;
        }
        self.ir_context.builder.build_br(&end_block);

        self.ir_context.builder.position_at_end(&end_block);
        self.ok_result()
    }

    fn walk_assignment(&self, assign: &crate::ast::Assignment) -> CompileResult {
        let id = match assign.identifiers.len() {
            1 => self.walk_identifier(&assign.identifiers[0])?,
            _ => {
                return Err(ASTLoweringError {
                    message: "not support multiple assign".to_string(),
                })
            }
        };

        let id = match id {
            cfg::Expr::Identifier(id) => id,
            _ => todo!(),
        };

        let val = self.walk_expr(&assign.value)?;

        if self.ret_var_has_init.borrow().contains_key(&id)
            && !self.ret_var_has_init.borrow().get(&id).unwrap()
        {
            let ty = self.get_id_type(id);
            self.ir_context
                .builder
                .build_declaration(id, Some(val), ty.clone());
        } else {
            self.ir_context.builder.build_assignment(id, val);
        }

        self.ok_result()
    }

    fn walk_variable_declaration(
        &self,
        var_decl: &crate::ast::VariableDeclaration,
    ) -> CompileResult {
        let (name, ty) = match var_decl.identifiers.len() {
            1 => (
                var_decl.identifiers[0].identifier.name.clone(),
                self.parse_ty_name(&var_decl.identifiers[0].type_name),
            ),
            _ => {
                return Err(ASTLoweringError {
                    message: "not support multiple decl".to_string(),
                })
            }
        };

        if let Some(value) = &var_decl.value {
            self.walk_expr(value);
        }

        let id = self.ir_context.builder.get_ident_id();
        self.vars.borrow_mut().insert(name, id);
        self.set_id_type(id, ty.clone());
        let init_val = match &var_decl.value {
            Some(val) => Some(self.walk_expr(val)?),
            None => None,
        };

        self.ir_context
            .builder
            .build_declaration(id.clone().into(), init_val, ty);
        self.ok_result()
    }

    fn walk_typed_identifier(&self, typed_id: &crate::ast::TypedIdentifier) -> CompileResult {
        self.walk_identifier(&typed_id.identifier);
        if let Some(ty) = &typed_id.type_name {
            self.walk_identifier(&ty.type_name);
        }
        self.ok_result()
    }

    pub(crate) fn walk_expr(&self, expr: &crate::ast::Expression) -> CompileResult {
        match expr {
            crate::ast::Expression::Identifier(id) => self.walk_identifier(id),
            crate::ast::Expression::FunctionCall(func_call) => self.walk_function_call(func_call),
            crate::ast::Expression::Literal(literal) => self.walk_literal(literal),
        }
    }

    fn walk_function_call(&self, func_call: &crate::ast::FunctionCall) -> CompileResult {
        let func_name = func_call.id.name.clone();
        if let Some(instr) = parse_intrinsic_func_name(&func_name) {
            return self.walk_yul_instruction(instr, func_call.arguments.as_slice());
        }
        let module_name = self.current_module_name.borrow().clone();
        let contract_name = self.current_contract_name.borrow().clone();
        let ret_ty = WORD_TY;
        let mut args = func_call
            .arguments
            .iter()
            .map(|arg| self.walk_expr(arg).unwrap())
            .collect();
        let qualifier_func_name = format!("{}.{}.{}", module_name, contract_name, func_name,);
        Ok(self
            .ir_context
            .builder
            .instr_call(qualifier_func_name.into(), args, ret_ty)
            .into())
    }

    fn walk_leave(&self) -> CompileResult {
        todo!()
    }

    fn walk_break(&self) -> CompileResult {
        todo!()
    }

    fn walk_continue(&self) -> CompileResult {
        todo!()
    }

    fn walk_identifier(&self, id: &crate::ast::Identifier) -> CompileResult {
        match self.vars.borrow().get(&id.name) {
            Some(id) => Ok(self.ir_context.builder.build_identifier(id)),
            None => Err(ASTLoweringError {
                message: format!("{:?} not found", id.name),
            }),
        }
    }

    fn walk_literal(&self, lit: &crate::ast::Literal) -> CompileResult {
        match lit {
            ast::Literal::TrueLiteral(ty_name) => Ok(self.bool_literal(&true)),
            ast::Literal::FalseLiteral(ty_name) => Ok(self.bool_literal(&false)),
            ast::Literal::HexNumberLiteral(hex, ty_name) => Ok(self.hex_literal(&hex.hex)),
            ast::Literal::DecimalNumberLiteral(dec, ty_name) => Ok(self.dec_literal(&dec.dec)),
            ast::Literal::StringLiteral(s, ty_name) => Ok(self.string_literal(&s.str)),
        }
    }

    fn ok_result(&self) -> CompileResult {
        Ok(self.ir_context.builder.build_nop())
    }
}

fn switch2ifelse(switch: &Switch) -> IfElse {
    let mut if_else_stmt = IfElse {
        cond: Expression::Literal(ast::Literal::TrueLiteral(None)),
        body: Block { statements: vec![] },
        else_body: Block { statements: vec![] },
    };
    let switch_cond = switch.condition.clone();
    match &switch.opt {
        ast::SwitchOptions::Cases(cases, default) => {
            if let Some(default) = default {
                if_else_stmt
                    .body
                    .statements
                    .extend(default.body.statements.clone());
            }

            for case in cases.into_iter().rev() {
                let cond = Expression::FunctionCall(Box::new(FunctionCall {
                    id: Identifier {
                        name: "eq".to_string(),
                    },
                    arguments: vec![switch_cond.clone(), Expression::Literal(case.case.clone())],
                }));
                let new_if = IfElse {
                    cond,
                    body: case.body.clone(),
                    else_body: Block {
                        statements: vec![Statement::IfElse(Box::new(if_else_stmt.clone()))],
                    },
                };
                if_else_stmt = new_if;
            }
        }
        ast::SwitchOptions::Default(_) => todo!(),
    }
    if_else_stmt
}

impl Yul2IRContext {
    fn parse_ty(&self, ty: &String) -> Type {
        match ty.as_str() {
            "bool" => Type::bool(),
            "u32" => Type::u32(),
            _ => Type::void(),
        }
    }

    fn parse_ty_name(&self, ty_name: &Option<TypeName>) -> Type {
        match ty_name {
            Some(ty) => self.parse_ty(&ty.type_name.name),
            // https://docs.soliditylang.org/en/latest/yul.html#motivation-and-high-level-description
            // Currently, there is only one specified dialect of Yul. This dialect uses the EVM opcodes as builtin functions
            // (see below) and defines only the type u256, which is the native 256-bit type of the EVM.
            // Because of that, we will not provide types in the examples below.
            None => WORD_TY,
        }
    }

    pub fn push_label_target(&self, label_target: Label) {
        self.ir_context
            .builder
            .context
            .labels
            .borrow_mut()
            .push(label_target);
    }

    pub fn pop_label_target(&self) {
        self.ir_context.builder.context.labels.borrow_mut().pop();
    }

    pub fn bool_literal(&self, v: &bool) -> Expr {
        Expr::Literal(Literal::Bool(*v))
    }

    pub fn string_literal(&self, v: &str) -> Expr {
        Expr::Literal(Literal::Str(v.to_owned()))
    }

    pub fn hex_literal(&self, v: &str) -> Expr {
        let without_prefix = v.trim_start_matches("0x");
        Expr::Literal(Literal::Int(cfg::IntLiteral::U256(
            BigInt::parse_bytes(without_prefix.as_bytes(), 16).unwrap(),
        )))
    }

    pub fn dec_literal(&self, v: &str) -> Expr {
        Expr::Literal(Literal::Int(cfg::IntLiteral::U256(
            BigInt::parse_bytes(v.as_bytes(), 10).unwrap(),
        )))
    }

    pub fn get_id_type(&self, id: IdentifierId) -> Type {
        self.ir_context
            .builder
            .context
            .current_function
            .borrow()
            .clone()
            .as_ref()
            .unwrap()
            .vars
            .get(&id)
            .unwrap()
            .clone()
    }

    pub fn set_id_type(&self, id: IdentifierId, ty: Type) {
        self.ir_context
            .builder
            .context
            .current_function
            .borrow_mut()
            .as_mut()
            .unwrap()
            .vars
            .insert(id, ty);
    }
}

impl Yul2IRContext {
    pub fn transform(&mut self) -> CompileResult {
        initialize_intrinisic_func_names();
        self.transform_object(&self.yul_ast.clone().unwrap(), true)?;
        self.ok_result()
    }

    pub fn transform_object(&mut self, object: &Object, is_main: bool) -> CompileResult {
        *self.current_module_name.borrow_mut() = object.name.clone();
        *self.current_contract_name.borrow_mut() = object.name.clone();

        let module_name = self.current_module_name.borrow().clone();
        let contract_name = self.current_contract_name.borrow().clone();
        if is_main {
            self.ir_context.main_module = module_name.clone();
        }
        let module = Module {
            name: module_name.clone(),
            types: IndexMap::new(),
            functions: IndexMap::new(),
            contract: None,
        };

        self.ir_context.current_module = RefCell::new(module);

        let mut c = Contract {
            name: contract_name.clone(),
            states: IndexMap::new(),
            functions: IndexMap::new(),
        };

        for func in object.code.statements.iter().filter(|stmt| match stmt {
            crate::ast::Statement::FunctionDefinition(_) => true,
            _ => false,
        }) {
            if let ast::Statement::FunctionDefinition(func_def) = func {
                let qualifier_func_name = format!(
                    "{}.{}.{}",
                    module_name,
                    contract_name,
                    func_def.name.name.clone()
                );
                let func_def = self.transform_func(func_def, qualifier_func_name)?;
                c.functions.insert(func_def.name.clone(), func_def);
            }
        }

        let init_def = self.transform_init_func(object)?;
        c.functions.insert("init".to_string(), init_def);

        let mut module = self.ir_context.current_module.borrow().clone();
        module.contract = Some(c);

        self.ir_context
            .modules
            .borrow_mut()
            .insert(module_name.clone(), module);

        for inner_segment in object.inner_segments.iter() {
            if let InnerSegment::Object(inner_object) = inner_segment {
                self.transform_object(inner_object, false)?;
            }
        }

        self.ok_result()
    }

    fn transform_init_func(
        &self,
        object: &Object,
    ) -> Result<smart_ir::ir::cfg::FunctionDefinition, ASTLoweringError> {
        let module_name = self.current_module_name.borrow().clone();
        let contract_name = self.current_contract_name.borrow().clone();
        let qualifier_func_name = format!("{}.{}.init", module_name, contract_name,);
        self.ir_context.builder.rest_ident_id();

        self.ir_context.builder.build_function(
            &qualifier_func_name,
            vec![],
            Type::void(),
            true,
            IndexMap::new(),
        );
        *self.data_id.borrow_mut() = self.declare_data_var(None, None);
        *self.caller_data_id.borrow_mut() = self.declare_data_var(None, None);
        *self.return_data_id.borrow_mut() = self.declare_data_var(None, None);

        for inner_segment in object.inner_segments.iter() {
            if let InnerSegment::Data(data) = inner_segment {
                for data_literal in data.iter() {
                    match data_literal {
                        ast::DataLiteral::HexLiteral(hex_literal) => {
                            let mut bytes = hex_literal.clone();
                            if hex_literal.len() % 2 != 0 {
                                bytes = bytes + "0"
                            };
                            let mut i = 0;
                            while i < bytes.len() {
                                self.ir_context.builder.build_call(
                                    get_intrinsic_func_by_key(IntrinsicFuncName::IR_VECTOR_PUSH)
                                        .unwrap()
                                        .into(),
                                    vec![
                                        self.ir_context
                                            .builder
                                            .build_identifier(&self.data_id.borrow()),
                                        self.ir_context
                                            .builder
                                            .instr_int_cast(
                                                self.hex_literal(&hex_literal[i..i + 2]),
                                                Type::u8(),
                                            )
                                            .into(),
                                    ],
                                    Type::void(),
                                );
                                i = i + 2;
                            }
                        }
                        ast::DataLiteral::StringLiteral(string_litreral) => {
                            for byte in string_litreral.as_bytes() {
                                self.ir_context.builder.build_call(
                                    get_intrinsic_func_by_key(IntrinsicFuncName::IR_VECTOR_PUSH)
                                        .unwrap()
                                        .into(),
                                    vec![
                                        self.ir_context
                                            .builder
                                            .build_identifier(&self.data_id.borrow()),
                                        self.ir_context
                                            .builder
                                            .instr_int_cast(
                                                self.dec_literal(&byte.to_string()),
                                                Type::u8(),
                                            )
                                            .into(),
                                    ],
                                    Type::void(),
                                )
                            }
                        }
                    }
                }
            }
        }

        for stmt in &object.code.statements {
            let _ = self.walk_stmt(stmt);
        }

        self.ir_context.builder.build_ret(None);
        self.ir_context.func_end();

        let func = self
            .ir_context
            .builder
            .context
            .current_function
            .borrow()
            .clone()
            .unwrap();
        Ok(func)
    }

    fn transform_func(
        &self,
        function: &FunctionDefinition,
        qualifier_func_name: String,
    ) -> Result<smart_ir::ir::cfg::FunctionDefinition, ASTLoweringError> {
        let (mut params, mut vars) = (vec![], IndexMap::new());
        self.ir_context.builder.rest_ident_id();
        for param in function.params.iter() {
            let param_name = param.identifier.name.clone();
            let ir_ty = self.parse_ty_name(&param.type_name);
            let id = self.ir_context.builder.get_ident_id();
            self.vars.borrow_mut().insert(param_name.clone(), id);
            params.push(ir_ty.clone());
            vars.insert(id, ir_ty.clone());
        }

        params.push(mem_data_ty());
        let id = self.ir_context.builder.get_ident_id();
        *self.data_id.borrow_mut() = id;
        vars.insert(id, mem_data_ty());

        params.push(mem_data_ty());
        let id = self.ir_context.builder.get_ident_id();
        *self.caller_data_id.borrow_mut() = id;
        vars.insert(id, mem_data_ty());

        params.push(mem_data_ty());
        let id = self.ir_context.builder.get_ident_id();
        *self.return_data_id.borrow_mut() = id;
        vars.insert(id, mem_data_ty());

        let ret = match function.returns.clone().len() {
            0 => Type::void(),
            1 => {
                let ret_ty = self.parse_ty_name(&function.returns[0].type_name);
                let id = self.ir_context.builder.get_ident_id();
                let ret_name = &function.returns[0].identifier.name;
                self.vars.borrow_mut().insert(ret_name.clone(), id);
                vars.insert(id, ret_ty.clone());
                self.ret_var_has_init.borrow_mut().insert(id, false);
                ret_ty
            }
            _ => {
                let mut tuple_ty = vec![];
                for ret_ty in function.returns.clone() {
                    let parsed_ret_ty = self.parse_ty_name(&ret_ty.type_name);
                    let id = self.ir_context.builder.get_ident_id();
                    let ret_name = &ret_ty.identifier.name;
                    self.vars.borrow_mut().insert(ret_name.clone(), id);
                    vars.insert(id, parsed_ret_ty.clone());
                    self.ret_var_has_init.borrow_mut().insert(id, false);
                    tuple_ty.push(parsed_ret_ty);
                }

                Type::Tuple(Rc::new(tuple_ty))
            }
        };

        let is_external = false;

        self.ir_context.builder.build_function(
            &qualifier_func_name,
            params,
            ret,
            is_external,
            vars,
        );

        for stmt in &function.body.statements {
            let _ = self.walk_stmt(stmt);
        }
        match function.returns.len() {
            0 => self.ir_context.builder.build_ret(None),
            1 => {
                let ret_name = &function.returns[0].identifier.name;
                let id = self.vars.borrow().get(ret_name).unwrap().clone();
                self.ir_context
                    .builder
                    .build_ret(Some(self.ir_context.builder.build_identifier(&id)))
            }
            _ => {
                self.ir_context.builder.build_call(
                    get_intrinsic_func_by_key(IntrinsicFuncName::IR_TUPLE_SET)
                        .unwrap()
                        .into(),
                    vec![],
                    cfg::Type::void(),
                );

                // TODO: support return tuple
                let ret_name = &function.returns[0].identifier.name;
                let id = self.vars.borrow().get(ret_name).unwrap().clone();
                self.ir_context
                    .builder
                    .build_ret(Some(self.ir_context.builder.build_identifier(&id)))
            }
        }
        self.ir_context.func_end();

        let func = self
            .ir_context
            .builder
            .context
            .current_function
            .borrow()
            .clone()
            .unwrap();
        Ok(func)
    }

    pub(crate) fn transform_int_to_signed(&self, expr: Expr, int_ty: Type) -> CompileResult {
        Ok(self.ir_context.builder.instr_int_cast(expr, int_ty).into())
    }

    pub(crate) fn int_max_literal(&self) -> Expr {
        self.ir_context
            .builder
            .instr_bit_not(self.hex_literal("0x0"))
            .into()
    }

    pub(crate) fn declare_int_var(&self, init_val: Option<Expr>) -> IdentifierId {
        let id = self.ir_context.builder.get_ident_id();
        self.set_id_type(id, WORD_TY);
        self.ir_context
            .builder
            .build_declaration(id.clone().into(), init_val, WORD_TY);
        id
    }

    pub(crate) fn declare_data_var(
        &self,
        init_val: Option<Expr>,
        len: Option<u32>,
    ) -> IdentifierId {
        let data_ty = Type::Array {
            elem: Rc::new(Type::u8()),
            len,
        };
        let id = self.ir_context.builder.get_ident_id();
        self.set_id_type(id, data_ty.clone());
        self.ir_context
            .builder
            .build_declaration(id.clone().into(), init_val, data_ty);
        if let Some(len) = len {
            self.build_data_expand(
                self.ir_context.builder.build_identifier(&id),
                self.dec_literal(&len.to_string()),
            );
        }
        id
    }

    pub(crate) fn build_data_load_byte(&self, data: cfg::Expr, offset: cfg::Expr) -> CompileResult {
        Ok(self
            .ir_context
            .builder
            .instr_int_cast(
                self.ir_context
                    .builder
                    .instr_call(
                        get_intrinsic_func_by_key(IntrinsicFuncName::IR_VECTOR_GET)
                            .unwrap()
                            .into(),
                        vec![data, offset],
                        cfg::Type::u8(),
                    )
                    .into(),
                WORD_TY,
            )
            .into())
    }

    pub(crate) fn build_data_load_word(&self, data: cfg::Expr, offset: cfg::Expr) -> CompileResult {
        let result_id = self.declare_int_var(Some(self.int_max_literal()));
        for i in 0..WORD_SIZE {
            let byte_offset = self.dec_literal(&i.to_string());
            let cur_offset = self
                .ir_context
                .builder
                .instr_add(offset.clone(), byte_offset.clone())
                .into();
            let byte_data = self.build_data_load_byte(data.clone(), cur_offset)?;

            self.ir_context.builder.build_assignment(
                result_id,
                self.ir_context
                    .builder
                    .instr_bit_and(
                        self.ir_context.builder.build_identifier(&result_id),
                        self.ir_context
                            .builder
                            .instr_lshift(
                                byte_data,
                                self.ir_context
                                    .builder
                                    .instr_mul(byte_offset, self.hex_literal("0x8"))
                                    .into(),
                            )
                            .into(),
                    )
                    .into(),
            );
        }
        Ok(self.ir_context.builder.build_identifier(&result_id))
    }

    pub(crate) fn build_data_store_byte(
        &self,
        data: cfg::Expr,
        offset: cfg::Expr,
        value: cfg::Expr,
    ) -> CompileResult {
        Ok(self
            .ir_context
            .builder
            .instr_call(
                get_intrinsic_func_by_key(IntrinsicFuncName::IR_VECTOR_SET)
                    .unwrap()
                    .into(),
                vec![
                    data,
                    offset,
                    self.ir_context
                        .builder
                        .instr_int_cast(
                            self.ir_context
                                .builder
                                .instr_and(value, self.hex_literal("0xff"))
                                .into(),
                            cfg::Type::u8(),
                        )
                        .into(),
                ],
                cfg::Type::void(),
            )
            .into())
    }

    pub(crate) fn build_data_expand(&self, data: cfg::Expr, capability: cfg::Expr) {
        let cur_size = self.declare_int_var(Some(
            self.ir_context
                .builder
                .instr_call(
                    get_intrinsic_func_by_key(IntrinsicFuncName::IR_VECTOR_LEN)
                        .unwrap()
                        .into(),
                    vec![data.clone()],
                    cfg::Type::i32(),
                )
                .into(),
        ));

        let cond_block = self.ir_context.append_block("cond");
        self.ir_context.builder.build_br(&cond_block);
        self.ir_context.builder.position_at_end(&cond_block);
        let cond_value = self
            .ir_context
            .builder
            .instr_lt(
                self.ir_context.builder.build_identifier(&cur_size),
                capability.clone(),
            )
            .into();
        let then_block = self.ir_context.append_block("body");
        let end_block = self.ir_context.append_block("exit");
        self.ir_context
            .builder
            .build_cond_br(cond_value, &then_block, &end_block);

        self.ir_context.builder.position_at_end(&then_block);

        self.ir_context.builder.build_call(
            get_intrinsic_func_by_key(IntrinsicFuncName::IR_VECTOR_PUSH)
                .unwrap()
                .into(),
            vec![cfg::Expr::Literal(cfg::Literal::Int(cfg::IntLiteral::U8(
                0,
            )))],
            cfg::Type::void(),
        );

        self.ir_context.builder.build_assignment(
            cur_size,
            self.ir_context
                .builder
                .instr_add(
                    self.ir_context.builder.build_identifier(&cur_size),
                    self.hex_literal("0x1"),
                )
                .into(),
        );
        self.ir_context.builder.build_br(&cond_block);
        self.ir_context.builder.position_at_end(&end_block);
    }

    pub(crate) fn build_data_store_word(
        &self,
        data: cfg::Expr,
        offset: cfg::Expr,
        value: cfg::Expr,
    ) -> CompileResult {
        for i in 0..WORD_SIZE {
            let byte_offset = self.dec_literal(&i.to_string());
            let cur_value = self
                .ir_context
                .builder
                .instr_rshift(value.clone(), byte_offset.clone())
                .into();
            let cur_offset = self
                .ir_context
                .builder
                .instr_add(offset.clone(), byte_offset)
                .into();
            self.build_data_store_byte(data.clone(), cur_offset, cur_value)?;
        }
        Ok(self.ir_context.builder.build_nop())
    }

    pub(crate) fn build_data_copy(
        &self,
        dst: cfg::Expr,
        src: cfg::Expr,
        dst_pos: cfg::Expr,
        src_pos: cfg::Expr,
        len: cfg::Expr,
    ) -> CompileResult {
        let offset = self.declare_int_var(Some(self.hex_literal("0x0")));

        let cond_block = self.ir_context.append_block("copy_cond");
        self.ir_context.builder.build_br(&cond_block);
        self.ir_context.builder.position_at_end(&cond_block);
        let cond_value = self
            .ir_context
            .builder
            .instr_lt(
                self.ir_context.builder.build_identifier(&offset),
                len.clone(),
            )
            .into();
        let then_block = self.ir_context.append_block("copy_body");
        let end_block = self.ir_context.append_block("copy_exit");
        self.ir_context
            .builder
            .build_cond_br(cond_value, &then_block, &end_block);

        self.ir_context.builder.position_at_end(&then_block);

        let byte_value = self.build_data_load_byte(
            src.clone(),
            self.ir_context
                .builder
                .instr_add(
                    src_pos.clone(),
                    self.ir_context.builder.build_identifier(&offset),
                )
                .into(),
        )?;

        self.build_data_store_byte(
            dst.clone(),
            self.ir_context
                .builder
                .instr_add(
                    dst_pos.clone(),
                    self.ir_context.builder.build_identifier(&offset),
                )
                .into(),
            byte_value,
        )?;

        self.ir_context.builder.build_assignment(
            offset,
            self.ir_context
                .builder
                .instr_add(
                    self.ir_context.builder.build_identifier(&offset),
                    self.hex_literal("0x1"),
                )
                .into(),
        );
        self.ir_context.builder.build_br(&cond_block);
        self.ir_context.builder.position_at_end(&end_block);
        Ok(self.ir_context.builder.build_nop())
    }

    pub(crate) fn build_log(
        &self,
        data: cfg::Expr,
        start_pos: cfg::Expr,
        len: cfg::Expr,
        topics: Vec<cfg::Expr>,
    ) -> CompileResult {
        let topic_array_ty = Type::Array {
            elem: Rc::new(mem_data_ty()),
            len: None,
        };
        let topic_array_id = self.ir_context.builder.get_ident_id();
        self.set_id_type(topic_array_id, topic_array_ty.clone());
        self.ir_context
            .builder
            .build_declaration(topic_array_id, None, topic_array_ty);
        for topic in topics {
            let topic_id = self.declare_data_var(None, Some(WORD_SIZE as u32));
            self.build_data_store_word(
                self.ir_context.builder.build_identifier(&topic_id),
                self.hex_literal("0x0"),
                topic,
            )?;
            self.ir_context.builder.build_call(
                get_intrinsic_func_by_key(IntrinsicFuncName::IR_VECTOR_PUSH)
                    .unwrap()
                    .into(),
                vec![
                    self.ir_context.builder.build_identifier(&topic_array_id),
                    self.ir_context.builder.build_identifier(&topic_id),
                ],
                cfg::Type::void(),
            )
        }

        let end_pos = self.ir_context.builder.instr_add(start_pos.clone(), len);
        let msg = self.ir_context.builder.instr_call(
            get_intrinsic_func_by_key(IntrinsicFuncName::IR_VECTOR_SLICE)
                .unwrap()
                .into(),
            vec![data, start_pos, end_pos.into()],
            mem_data_ty(),
        );

        Ok(self
            .ir_context
            .builder
            .instr_call(
                get_intrinsic_func_by_key(IntrinsicFuncName::IR_BUILTIN_CALL_LOG)
                    .unwrap()
                    .into(),
                vec![
                    self.ir_context.builder.build_identifier(&topic_array_id),
                    msg.into(),
                ],
                cfg::Type::void(),
            )
            .into())
    }
}
