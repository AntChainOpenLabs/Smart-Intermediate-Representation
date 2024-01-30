// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use smart_ir::ir::{
    cfg,
    context::{ASTLoweringError, CompileResult},
    interface_type::{get_intrinsic_func_by_key, IntrinsicFuncName},
};

use crate::{
    ast::Expression,
    context::{mem_data_ty, Yul2IRContext, SIGNED_WORD_TY, WORD_SIZE, WORD_TY},
};


#[derive(Debug, Clone)]
pub enum YulInstructionName {
    Stop,
    Add,
    Sub,
    Mul,
    Div,
    SDiv,
    Mod,
    SMod,
    Exp,
    Not,
    Lt,
    Gt,
    SLt,
    SGt,
    Eq,
    IsZero,
    And,
    Or,
    Xor,
    Byte,
    Shl,
    Shr,
    Sar,
    AddMod,
    MulMod,
    SignExtend,
    Keccak256,
    PC,
    Pop,
    MLoad,
    MStore,
    MStore8,
    SLoad,
    SStore,
    MSize,
    Gas,
    Address,
    Balance,
    SelfBalance,
    Caller,
    CallValue,
    CallDataLoad,
    CallDataSize,
    CallDataCopy,
    CodeSize,
    CodeCopy,
    ExtCodeSize,
    ExtCodeCopy,
    DataCopy,
    DataOffset,
    DataSize,
    ReturnDataSize,
    ReturnDataCopy,
    ExtCodeHash,
    Create,
    Create2,
    Call,
    CallCode,
    DelegateCall,
    StaticCall,
    Return,
    Revert,
    SelfDestruct,
    Invalid,
    Log0,
    Log1,
    Log2,
    Log3,
    Log4,
    ChainID,
    BaseFee,
    Origin,
    GasPrice,
    BlockHash,
    CoinBase,
    TimeStamp,
    Number,
    Difficulty,
    Prevrandao,
    GasLimit,
}

impl From<String> for YulInstructionName {
    fn from(s: String) -> YulInstructionName {
        match parse_intrinsic_func_name(&s) {
            Some(intrinsic) => intrinsic,
            None => unimplemented!("instruction {} unimplemented", s),
        }
    }
}

pub(crate) fn parse_intrinsic_func_name(s: &str) -> Option<YulInstructionName> {
    match s {
        "stop" => Some(YulInstructionName::Stop),
        "add" => Some(YulInstructionName::Add),
        "sub" => Some(YulInstructionName::Sub),
        "mul" => Some(YulInstructionName::Mul),
        "div" => Some(YulInstructionName::Div),
        "sdiv" => Some(YulInstructionName::SDiv),
        "mod" => Some(YulInstructionName::Mod),
        "smod" => Some(YulInstructionName::SMod),
        "exp" => Some(YulInstructionName::Exp),
        "not" => Some(YulInstructionName::Not),
        "lt" => Some(YulInstructionName::Lt),
        "gt" => Some(YulInstructionName::Gt),
        "slt" => Some(YulInstructionName::SLt),
        "sgt" => Some(YulInstructionName::SGt),
        "eq" => Some(YulInstructionName::Eq),
        "iszero" => Some(YulInstructionName::IsZero),
        "and" => Some(YulInstructionName::And),
        "or" => Some(YulInstructionName::Or),
        "xor" => Some(YulInstructionName::Xor),
        "byte" => Some(YulInstructionName::Byte),
        "shl" => Some(YulInstructionName::Shl),
        "shr" => Some(YulInstructionName::Shr),
        "sar" => Some(YulInstructionName::Sar),
        "addmod" => Some(YulInstructionName::AddMod),
        "mulmod" => Some(YulInstructionName::MulMod),
        "signextend" => Some(YulInstructionName::SignExtend),
        "keccak256" => Some(YulInstructionName::Keccak256),
        "pc" => Some(YulInstructionName::PC),
        "pop" => Some(YulInstructionName::Pop),
        "mload" => Some(YulInstructionName::MLoad),
        "mstore" => Some(YulInstructionName::MStore),
        "mstore8" => Some(YulInstructionName::MStore8),
        "sload" => Some(YulInstructionName::SLoad),
        "sstore" => Some(YulInstructionName::SStore),
        "msize" => Some(YulInstructionName::MSize),
        "gas" => Some(YulInstructionName::Gas),
        "address" => Some(YulInstructionName::Address),
        "balance" => Some(YulInstructionName::Balance),
        "selfbalance" => Some(YulInstructionName::SelfBalance),
        "caller" => Some(YulInstructionName::Caller),
        "callvalue" => Some(YulInstructionName::CallValue),
        "calldataload" => Some(YulInstructionName::CallDataLoad),
        "calldatasize" => Some(YulInstructionName::CallDataSize),
        "calldatacopy" => Some(YulInstructionName::CallDataCopy),
        "codesize" => Some(YulInstructionName::CodeSize),
        "codecopy" => Some(YulInstructionName::CodeCopy),
        "extcodesize" => Some(YulInstructionName::ExtCodeSize),
        "extcodecopy" => Some(YulInstructionName::ExtCodeCopy),
        "datacopy" => Some(YulInstructionName::DataCopy),
        "dataoffset" => Some(YulInstructionName::DataOffset),
        "datasize" => Some(YulInstructionName::DataSize),
        "returndatasize" => Some(YulInstructionName::ReturnDataSize),
        "returndatacopy" => Some(YulInstructionName::ReturnDataCopy),
        "extcodehash" => Some(YulInstructionName::ExtCodeHash),
        "create" => Some(YulInstructionName::Create),
        "create2" => Some(YulInstructionName::Create2),
        "call" => Some(YulInstructionName::Call),
        "callcode" => Some(YulInstructionName::CallCode),
        "delegatecall" => Some(YulInstructionName::DelegateCall),
        "staticcall" => Some(YulInstructionName::StaticCall),
        "return" => Some(YulInstructionName::Return),
        "revert" => Some(YulInstructionName::Revert),
        "selfdestruct" => Some(YulInstructionName::SelfDestruct),
        "invalid" => Some(YulInstructionName::Invalid),
        "log0" => Some(YulInstructionName::Log0),
        "log1" => Some(YulInstructionName::Log1),
        "log2" => Some(YulInstructionName::Log2),
        "log3" => Some(YulInstructionName::Log3),
        "log4" => Some(YulInstructionName::Log4),
        "chainid" => Some(YulInstructionName::ChainID),
        "basefee" => Some(YulInstructionName::BaseFee),
        "origin" => Some(YulInstructionName::Origin),
        "gasprice" => Some(YulInstructionName::GasPrice),
        "blockhash" => Some(YulInstructionName::BlockHash),
        "coinbase" => Some(YulInstructionName::CoinBase),
        "timestamp" => Some(YulInstructionName::TimeStamp),
        "number" => Some(YulInstructionName::Number),
        "difficulty" => Some(YulInstructionName::Difficulty),
        "prevrandao" => Some(YulInstructionName::Prevrandao),
        "gaslimit" => Some(YulInstructionName::GasLimit),
        _ => None,
    }
}

impl Yul2IRContext {
    pub(crate) fn walk_yul_instruction(
        &self,
        instr: YulInstructionName,
        args: &[Expression],
    ) -> CompileResult {
        let args = args
            .iter()
            .map(|arg| self.walk_expr(arg).unwrap())
            .collect();
        match instr {
            YulInstructionName::Stop => Ok(self
                .ir_context
                .builder
                .instr_call(
                    get_intrinsic_func_by_key(IntrinsicFuncName::IR_BUILTIN_ABORT)
                        .unwrap()
                        .into(),
                    args,
                    cfg::Type::void(),
                )
                .into()),
            YulInstructionName::Add => Ok(self
                .ir_context
                .builder
                .instr_add(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"add\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"add\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::Sub => Ok(self
                .ir_context
                .builder
                .instr_sub(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"sub\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"sub\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::Mul => Ok(self
                .ir_context
                .builder
                .instr_mul(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"mul\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"mul\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::Div => Ok(self
                .ir_context
                .builder
                .instr_div(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"div\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"div\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::SDiv => Ok(self
                .ir_context
                .builder
                .instr_div(
                    self.transform_int_to_signed(
                        args.get(0)
                            .ok_or(ASTLoweringError {
                                message: "the number of args of instr \"sdiv\" is less than 2 "
                                    .to_string(),
                            })
                            .cloned()?,
                        SIGNED_WORD_TY,
                    )?,
                    self.transform_int_to_signed(
                        args.get(1)
                            .ok_or(ASTLoweringError {
                                message: "the number of args of instr \"sdiv\" is less than 2 "
                                    .to_string(),
                            })
                            .cloned()?,
                        SIGNED_WORD_TY,
                    )?,
                )
                .into()),
            YulInstructionName::Mod => Ok(self
                .ir_context
                .builder
                .instr_mod(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"mod\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"mod\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::SMod => Ok(self
                .ir_context
                .builder
                .instr_mod(
                    self.transform_int_to_signed(
                        args.get(0)
                            .ok_or(ASTLoweringError {
                                message: "the number of args of instr \"smod\" is less than 2 "
                                    .to_string(),
                            })
                            .cloned()?,
                        SIGNED_WORD_TY,
                    )?,
                    self.transform_int_to_signed(
                        args.get(1)
                            .ok_or(ASTLoweringError {
                                message: "the number of args of instr \"smod\" is less than 2 "
                                    .to_string(),
                            })
                            .cloned()?,
                        SIGNED_WORD_TY,
                    )?,
                )
                .into()),
            YulInstructionName::Exp => Ok(self
                .ir_context
                .builder
                .instr_pow(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"exp\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"exp\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::Not => Ok(self
                .ir_context
                .builder
                .instr_bit_not(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"not\" is less than 1 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::Lt => Ok(self
                .ir_context
                .builder
                .instr_lt(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"lt\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"lt\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::Gt => Ok(self
                .ir_context
                .builder
                .instr_gt(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"gt\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"gt\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::SLt => Ok(self
                .ir_context
                .builder
                .instr_lt(
                    self.transform_int_to_signed(
                        args.get(0)
                            .ok_or(ASTLoweringError {
                                message: "the number of args of instr \"slt\" is less than 2 "
                                    .to_string(),
                            })
                            .cloned()?,
                        SIGNED_WORD_TY,
                    )?,
                    self.transform_int_to_signed(
                        args.get(1)
                            .ok_or(ASTLoweringError {
                                message: "the number of args of instr \"slt\" is less than 2 "
                                    .to_string(),
                            })
                            .cloned()?,
                        SIGNED_WORD_TY,
                    )?,
                )
                .into()),
            YulInstructionName::SGt => Ok(self
                .ir_context
                .builder
                .instr_gt(
                    self.transform_int_to_signed(
                        args.get(0)
                            .ok_or(ASTLoweringError {
                                message: "the number of args of instr \"sgt\" is less than 2 "
                                    .to_string(),
                            })
                            .cloned()?,
                        SIGNED_WORD_TY,
                    )?,
                    self.transform_int_to_signed(
                        args.get(1)
                            .ok_or(ASTLoweringError {
                                message: "the number of args of instr \"sgt\" is less than 2 "
                                    .to_string(),
                            })
                            .cloned()?,
                        SIGNED_WORD_TY,
                    )?,
                )
                .into()),
            YulInstructionName::Eq => Ok(self
                .ir_context
                .builder
                .instr_eq(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"eq\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"eq\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::IsZero => Ok(self
                .ir_context
                .builder
                .instr_eq(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"iszero\" is less than 1 "
                                .to_string(),
                        })
                        .cloned()?,
                    self.hex_literal("0x0"),
                )
                .into()),
            YulInstructionName::And => Ok(self
                .ir_context
                .builder
                .instr_bit_and(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"and\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"and\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::Or => Ok(self
                .ir_context
                .builder
                .instr_bit_or(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"or\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"or\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::Xor => Ok(self
                .ir_context
                .builder
                .instr_bit_xor(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"xor\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"xor\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::Byte => {
                let nth_byte = args
                    .get(0)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"byte\" is less than 2 ".to_string(),
                    })
                    .cloned()?;
                let value = args
                    .get(1)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"byte\" is less than 2 ".to_string(),
                    })
                    .cloned()?;
                let nth_bit = self
                    .ir_context
                    .builder
                    .instr_mul(nth_byte, self.hex_literal("0x8"));
                let moved_byte = self.ir_context.builder.instr_rshift(value, nth_bit.into());
                Ok(self
                    .ir_context
                    .builder
                    .instr_bit_and(moved_byte.into(), self.hex_literal("0x1"))
                    .into())
            }
            YulInstructionName::Shl => Ok(self
                .ir_context
                .builder
                .instr_lshift(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"shl\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"shl\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::Shr => Ok(self
                .ir_context
                .builder
                .instr_rshift(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"shr\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"shr\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::Sar => Ok(cfg::Instr::new(
                cfg::InstrDescription::sar(
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"shr\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                    args.get(1)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"shr\" is less than 2 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into(),
            )
            .into()),
            YulInstructionName::AddMod => Ok(self
                .ir_context
                .builder
                .instr_mod(
                    self.ir_context
                        .builder
                        .instr_add(
                            args.get(0)
                                .ok_or(ASTLoweringError {
                                    message:
                                        "the number of args of instr \"addmod\" is less than 3 "
                                            .to_string(),
                                })
                                .cloned()?,
                            args.get(1)
                                .ok_or(ASTLoweringError {
                                    message:
                                        "the number of args of instr \"addmod\" is less than 3 "
                                            .to_string(),
                                })
                                .cloned()?,
                        )
                        .into(),
                    args.get(2)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"addmod\" is less than 3 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::MulMod => Ok(self
                .ir_context
                .builder
                .instr_mod(
                    self.ir_context
                        .builder
                        .instr_mul(
                            args.get(0)
                                .ok_or(ASTLoweringError {
                                    message:
                                        "the number of args of instr \"mulmod\" is less than 3 "
                                            .to_string(),
                                })
                                .cloned()?,
                            args.get(1)
                                .ok_or(ASTLoweringError {
                                    message:
                                        "the number of args of instr \"mulmod\" is less than 3 "
                                            .to_string(),
                                })
                                .cloned()?,
                        )
                        .into(),
                    args.get(2)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"mulmod\" is less than 3 "
                                .to_string(),
                        })
                        .cloned()?,
                )
                .into()),
            YulInstructionName::Keccak256 => {
                let input_data = self.declare_data_var(None, Some(WORD_SIZE as u32));
                self.build_data_store_byte(
                    self.ir_context.builder.build_identifier(&input_data),
                    self.hex_literal("0x0"),
                    args.get(0)
                        .ok_or(ASTLoweringError {
                            message: "the number of args of instr \"keccak256\" is less than 1 "
                                .to_string(),
                        })
                        .cloned()?,
                )?;
                let bytes = self.ir_context.builder.instr_call(
                    get_intrinsic_func_by_key(IntrinsicFuncName::IR_BUILTIN_KECCAK256)
                        .unwrap()
                        .into(),
                    vec![self.ir_context.builder.build_identifier(&input_data)],
                    cfg::Type::Array {
                        elem: Rc::new(cfg::Type::u8()),
                        len: None,
                    },
                );

                self.build_data_load_word(bytes.into(), self.hex_literal("0x0"))
            }
            YulInstructionName::Pop => Ok(self.ir_context.builder.build_nop()), //Do nothing
            YulInstructionName::MLoad => {
                let data = self
                    .ir_context
                    .builder
                    .build_identifier(&self.data_id.borrow());
                let offset = args
                    .get(0)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"mstore\" is less than 2 "
                            .to_string(),
                    })
                    .cloned()?;
                self.build_data_load_word(data, offset)
            }
            YulInstructionName::MStore => {
                let data = self
                    .ir_context
                    .builder
                    .build_identifier(&self.data_id.borrow());
                let offset = args
                    .get(0)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"mstore\" is less than 2 "
                            .to_string(),
                    })
                    .cloned()?;
                let value = args
                    .get(1)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"mstore\" is less than 2 "
                            .to_string(),
                    })
                    .cloned()?;
                self.build_data_store_word(data, offset, value)
            }
            YulInstructionName::MStore8 => {
                let data = self
                    .ir_context
                    .builder
                    .build_identifier(&self.data_id.borrow());
                let offset = args
                    .get(0)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"mstore8\" is less than 2 "
                            .to_string(),
                    })
                    .cloned()?;
                let value = args
                    .get(1)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"mstore8\" is less than 2 "
                            .to_string(),
                    })
                    .cloned()?;
                self.build_data_store_byte(data, offset, value)
            }
            YulInstructionName::SLoad => {
                let storage_path = self.ir_context.builder.instr_get_storage_path(args);
                Ok(self
                    .ir_context
                    .builder
                    .instr_storage_load(storage_path.into(), WORD_TY)
                    .into())
            }
            YulInstructionName::SStore => {
                let pos = args
                    .get(0)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"sstore\" is less than 2 "
                            .to_string(),
                    })
                    .cloned()?;
                let value = args
                    .get(1)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"sstore\" is less than 2 "
                            .to_string(),
                    })
                    .cloned()?;
                let storage_path = self.ir_context.builder.instr_get_storage_path(vec![pos]);
                Ok(self
                    .ir_context
                    .builder
                    .instr_storage_store(storage_path.into(), value)
                    .into())
            }
            YulInstructionName::MSize => Ok(self
                .ir_context
                .builder
                .instr_div(
                    self.ir_context
                        .builder
                        .instr_call(
                            get_intrinsic_func_by_key(IntrinsicFuncName::IR_VECTOR_LEN)
                                .unwrap()
                                .into(),
                            vec![self
                                .ir_context
                                .builder
                                .build_identifier(&self.data_id.borrow())],
                            WORD_TY,
                        )
                        .into(),
                    self.hex_literal(&WORD_SIZE.to_string()),
                )
                .into()),
            YulInstructionName::Caller => {
                let caller_bytes = self.ir_context.builder.instr_call(
                    get_intrinsic_func_by_key(IntrinsicFuncName::IR_STR_TO_BYTES)
                        .unwrap()
                        .into(),
                    vec![self
                        .ir_context
                        .builder
                        .instr_call(
                            get_intrinsic_func_by_key(IntrinsicFuncName::IR_BUILTIN_CALL_SENDER)
                                .unwrap()
                                .into(),
                            vec![],
                            cfg::Type::str(),
                        )
                        .into()],
                    mem_data_ty(),
                );
                let caller_bytes_id = self.declare_int_var(Some(caller_bytes.into()));

                self.build_data_load_word(
                    self.ir_context.builder.build_identifier(&caller_bytes_id),
                    self.hex_literal("0x0"),
                )
            }
            YulInstructionName::CallValue => Ok(self
                .ir_context
                .builder
                .instr_call(
                    get_intrinsic_func_by_key(IntrinsicFuncName::IR_BUILTIN_CALL_GAS_LEFT)
                        .unwrap()
                        .into(),
                    vec![],
                    WORD_TY,
                )
                .into()),
            YulInstructionName::CallDataLoad => {
                let data = self
                    .ir_context
                    .builder
                    .build_identifier(&self.caller_data_id.borrow());
                let offset = args
                    .get(0)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"calldataload\" is less than 2 "
                            .to_string(),
                    })
                    .cloned()?;
                self.build_data_load_word(data, offset)
            }
            YulInstructionName::CallDataSize => Ok(self
                .ir_context
                .builder
                .instr_div(
                    self.ir_context
                        .builder
                        .instr_call(
                            get_intrinsic_func_by_key(IntrinsicFuncName::IR_VECTOR_LEN)
                                .unwrap()
                                .into(),
                            vec![self
                                .ir_context
                                .builder
                                .build_identifier(&self.caller_data_id.borrow())],
                            WORD_TY,
                        )
                        .into(),
                    self.hex_literal(&WORD_SIZE.to_string()),
                )
                .into()),
            YulInstructionName::CallDataCopy => {
                let dst = self
                    .ir_context
                    .builder
                    .build_identifier(&self.data_id.borrow());
                let src = self
                    .ir_context
                    .builder
                    .build_identifier(&self.caller_data_id.borrow());

                let dst_pos = args
                    .get(0)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"calldatacopy\" is less than 3 "
                            .to_string(),
                    })
                    .cloned()?;
                let src_pos = args
                    .get(1)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"calldatacopy\" is less than 3 "
                            .to_string(),
                    })
                    .cloned()?;
                let len = args
                    .get(2)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"calldatacopy\" is less than 3 "
                            .to_string(),
                    })
                    .cloned()?;
                self.build_data_copy(dst, src, dst_pos, src_pos, len)
            }
            YulInstructionName::DataCopy => {
                let dst = self
                    .ir_context
                    .builder
                    .build_identifier(&self.data_id.borrow());
                let src = args
                    .get(1)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"datacopy\" is less than 3 "
                            .to_string(),
                    })
                    .cloned()?;
                let dst_pos = args
                    .get(0)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"datacopy\" is less than 3 "
                            .to_string(),
                    })
                    .cloned()?;
                let src_pos = self.hex_literal("0x0");
                let len = args
                    .get(2)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"datacopy\" is less than 3 "
                            .to_string(),
                    })
                    .cloned()?;
                self.build_data_copy(dst, src, dst_pos, src_pos, len)
            }
            YulInstructionName::DataOffset => {
                let address = args
                    .get(0)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"datasize\" is less than 1 "
                            .to_string(),
                    })
                    .cloned()?;

                Ok(self
                    .ir_context
                    .builder
                    .instr_call(
                        get_intrinsic_func_by_key(IntrinsicFuncName::IR_STR_TO_BYTES)
                            .unwrap()
                            .into(),
                        vec![address],
                        WORD_TY,
                    )
                    .into())
            }
            YulInstructionName::DataSize => {
                let address = args
                    .get(0)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"datasize\" is less than 1 "
                            .to_string(),
                    })
                    .cloned()?;
                Ok(self
                    .ir_context
                    .builder
                    .instr_call(
                        get_intrinsic_func_by_key(IntrinsicFuncName::IR_STR_LEN)
                            .unwrap()
                            .into(),
                        vec![address],
                        WORD_TY,
                    )
                    .into())
            }
            YulInstructionName::ReturnDataSize => Ok(self
                .ir_context
                .builder
                .instr_div(
                    self.ir_context
                        .builder
                        .instr_call(
                            get_intrinsic_func_by_key(IntrinsicFuncName::IR_VECTOR_LEN)
                                .unwrap()
                                .into(),
                            vec![self
                                .ir_context
                                .builder
                                .build_identifier(&self.return_data_id.borrow())],
                            WORD_TY,
                        )
                        .into(),
                    self.hex_literal(&WORD_SIZE.to_string()),
                )
                .into()),
            YulInstructionName::ReturnDataCopy => {
                let dst = self
                    .ir_context
                    .builder
                    .build_identifier(&self.data_id.borrow());
                let src = self
                    .ir_context
                    .builder
                    .build_identifier(&self.return_data_id.borrow());

                let dst_pos = args
                    .get(0)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"returndatacopy\" is less than 3 "
                            .to_string(),
                    })
                    .cloned()?;
                let src_pos = args
                    .get(1)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"returndatacopy\" is less than 3 "
                            .to_string(),
                    })
                    .cloned()?;
                let len = args
                    .get(2)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"returndatacopy\" is less than 3 "
                            .to_string(),
                    })
                    .cloned()?;
                self.build_data_copy(dst, src, dst_pos, src_pos, len)
            }
            YulInstructionName::Return =>
            //TODO : YulIntrinsicFunction
            {
                let data = self
                    .ir_context
                    .builder
                    .build_identifier(&self.data_id.borrow());

                let start_offset = args
                    .get(0)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"return\" is less than 2 "
                            .to_string(),
                    })
                    .cloned()?;
                let len = args
                    .get(1)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"return\" is less than 2 "
                            .to_string(),
                    })
                    .cloned()?;

                let end_offset = self.ir_context.builder.instr_add(start_offset.clone(), len);
                self.ir_context.builder.build_ret(Some(
                    self.ir_context
                        .builder
                        .instr_call(
                            get_intrinsic_func_by_key(IntrinsicFuncName::IR_VECTOR_SLICE)
                                .unwrap()
                                .into(),
                            vec![data, start_offset, end_offset.into()],
                            mem_data_ty(),
                        )
                        .into(),
                ));
                Ok(self.ir_context.builder.build_nop())
            }
            YulInstructionName::Revert => {
                self.ir_context.builder.build_call(
                    get_intrinsic_func_by_key(IntrinsicFuncName::IR_BUILTIN_REVERT)
                        .unwrap()
                        .into(),
                    vec![
                        cfg::Expr::Literal(cfg::Literal::Int(cfg::IntLiteral::I32(3001))),
                        self.string_literal("yul revert"),
                    ],
                    cfg::Type::void(),
                );
                Ok(self.ir_context.builder.build_nop())
            }
            YulInstructionName::Log3 => {
                let pos = args
                    .get(0)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"return\" is less than 5 "
                            .to_string(),
                    })
                    .cloned()?;
                let len = args
                    .get(1)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"return\" is less than 5 "
                            .to_string(),
                    })
                    .cloned()?;
                let t1 = args
                    .get(2)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"return\" is less than 5 "
                            .to_string(),
                    })
                    .cloned()?;
                let t2 = args
                    .get(3)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"return\" is less than 5 "
                            .to_string(),
                    })
                    .cloned()?;
                let t3 = args
                    .get(4)
                    .ok_or(ASTLoweringError {
                        message: "the number of args of instr \"return\" is less than 5 "
                            .to_string(),
                    })
                    .cloned()?;

                self.build_log(
                    self.ir_context
                        .builder
                        .build_identifier(&self.data_id.borrow()),
                    pos,
                    len,
                    vec![t1, t2, t3],
                )
            }
            instr_name => return Err(ASTLoweringError {
                message: format!("Internal failed: {:?} not implemented", instr_name)
            })
        }
    }
}
