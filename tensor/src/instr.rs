use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use smart_ir::ir::{
    builder::BasicBlockId,
    cfg::{BasicBlock, BinaryOp, CmpOp, ControlFlowGraph, Expr, Instr},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NodeFeat {
    pub idx_in_parent: u32,
    pub kind: String,
    pub inner_kind: String,
    pub depth: u32,
    pub children: Vec<NodeFeat>,
}

pub fn get_cfg_feats(cfg: &ControlFlowGraph) -> Vec<NodeFeat> {
    // let mut cfg_feats = HashMap::new();
    let mut cfg_feats = vec![];
    for (id, bb) in &cfg.basic_blocks {
        let bb_feats = get_bb_feats(bb, 1);

        // cfg_feats.insert(*id, bb_feats);
        cfg_feats.push(NodeFeat {
            idx_in_parent: *id,
            kind: "BasicBlock".to_string(),
            inner_kind: "BasicBlock".to_string(),
            depth: 0,
            children: bb_feats,
        })
    }
    cfg_feats
}

pub fn get_bb_feats(bb: &BasicBlock, depth: u32) -> Vec<NodeFeat> {
    let mut res = vec![];
    for (idx, instr) in bb.instrs.iter().enumerate() {
        res.push(walk_instr(instr, idx as u32, depth));
    }
    res
}

pub fn walk_instr(instr: &Instr, idx: u32, depth: u32) -> NodeFeat {
    let kind = InstrDescription2Idx(instr);
    let mut children = vec![];
    match &instr.inner {
        smart_ir::ir::cfg::InstrDescription::Declaration { id, init_val, ty } => {
            if let Some(val) = init_val {
                children.push(walk_expr(val, 0, depth + 1));
            }
        }
        smart_ir::ir::cfg::InstrDescription::Assignment { id, val } => {
            children.push(walk_expr(val, 0, depth + 1));
        }
        smart_ir::ir::cfg::InstrDescription::Ret { val } => {
            if let Some(val) = val {
                children.push(walk_expr(val, 0, depth + 1));
            }
        }
        smart_ir::ir::cfg::InstrDescription::Br { target } => {}
        smart_ir::ir::cfg::InstrDescription::BrIf {
            cond,
            then_bb,
            else_bb,
        } => {
            children.push(walk_expr(cond, 0, depth + 1));
        }
        smart_ir::ir::cfg::InstrDescription::Match {
            val,
            otherwise,
            jump_table,
        } => {
            children.push(walk_expr(val, 0, depth + 1));
        }
        smart_ir::ir::cfg::InstrDescription::Not { op } => {
            children.push(walk_expr(op, 0, depth + 1));
        }
        smart_ir::ir::cfg::InstrDescription::BitNot { op } => {
            children.push(walk_expr(op, 0, depth + 1))
        }
        smart_ir::ir::cfg::InstrDescription::Binary { op_code, op1, op2 } => {
            children.push(NodeFeat {
                idx_in_parent: 0,
                kind: "BinaryOp".to_string(),
                inner_kind: BinaryOp2Idx(op_code),
                depth: depth + 1,
                children: vec![],
            });
            children.push(walk_expr(op1, 1, depth + 1));
            children.push(walk_expr(op2, 2, depth + 1));
        }
        smart_ir::ir::cfg::InstrDescription::Cmp { op_code, op1, op2 } => {
            children.push(NodeFeat {
                idx_in_parent: 0,
                kind: "CmpOp".to_string(),
                inner_kind: CmpOp2Idx(op_code),
                depth: depth + 1,
                children: vec![],
            });
            children.push(walk_expr(op1, 1, depth + 1));
            children.push(walk_expr(op2, 2, depth + 1));
        }
        smart_ir::ir::cfg::InstrDescription::Alloca { ty } => {}
        smart_ir::ir::cfg::InstrDescription::Malloc { ty } => {}
        smart_ir::ir::cfg::InstrDescription::Free { ptr } => {
            children.push(walk_expr(ptr, 0, depth + 1))
        }
        smart_ir::ir::cfg::InstrDescription::GetField {
            ptr,
            field_path,
            field_ty,
        } => children.push(walk_expr(ptr, 0, depth + 1)),
        smart_ir::ir::cfg::InstrDescription::SetField {
            ptr,
            val,
            field_path,
        } => {
            children.push(walk_expr(ptr, 0, depth + 1));
            children.push(walk_expr(val, 1, depth + 1));
            // children.push(walk_expr(ptr, 2, depth + 1));
        }
        smart_ir::ir::cfg::InstrDescription::GetStoragePath { storage_path } => {
            for (i, path) in storage_path.iter().enumerate() {
                children.push(walk_expr(path, i as u32, depth + 1))
            }
        }
        smart_ir::ir::cfg::InstrDescription::StorageLoad {
            storage_path,
            load_ty,
        } => children.push(walk_expr(storage_path, 0, depth + 1)),
        smart_ir::ir::cfg::InstrDescription::StorageStore {
            storage_path,
            store_val,
        } => {
            children.push(walk_expr(storage_path, 0, depth + 1));
            children.push(walk_expr(store_val, 1, depth + 1));
        }
        smart_ir::ir::cfg::InstrDescription::Call {
            func_name,
            args,
            ret_ty,
        } => {
            children.push(NodeFeat {
                idx_in_parent: 0,
                kind: "PartialFuncName".to_string(),
                inner_kind: func_name.get_name(),
                depth: depth + 1,
                children: vec![],
            });
            let mut args_node = NodeFeat {
                idx_in_parent: 1,
                kind: "Vector".to_string(),
                inner_kind: "Args".to_string(),
                depth: depth + 1,
                children: vec![],
            };
            for arg in args {
                args_node.children.push(walk_expr(arg, 1, depth + 2));
            }
            children.push(args_node);
        }
        smart_ir::ir::cfg::InstrDescription::IntCast { val, target_ty } => {
            children.push(walk_expr(val, 0, depth + 1));
        }
    }
    NodeFeat {
        idx_in_parent: idx,
        inner_kind: kind,
        depth,
        children,
        kind: "Instr".to_string(),
    }
}

pub fn walk_expr(expr: &Expr, idx: u32, depth: u32) -> NodeFeat {
    let mut children = vec![];
    let inner_kind = Expr2Idx(expr);
    match expr {
        Expr::Instr(instr) => {
            children.push(walk_instr(instr, 0, depth + 1));
        }
        _ => {}
    }
    NodeFeat {
        idx_in_parent: idx,
        inner_kind,
        depth,
        children,
        kind: "Expr".to_string(),
    }
}

fn Expr2Idx(expr: &Expr) -> String {
    match expr {
        Expr::Identifier(_) => "Expr::Identifier".to_string(),
        Expr::Instr(_) => "Expr::Instr".to_string(),
        Expr::Literal(_) => "Expr::Literal".to_string(),
        Expr::NOP => "Expr::NOP".to_string(),
    }
}

fn BinaryOp2Idx(op: &BinaryOp) -> String {
    match op {
        BinaryOp::Add => "BinaryOp::Add".to_string(),
        BinaryOp::Sub => "BinaryOp::Sub".to_string(),
        BinaryOp::Mul => "BinaryOp::Mul".to_string(),
        BinaryOp::Div => "BinaryOp::Div".to_string(),
        BinaryOp::Mod => "BinaryOp::Mod".to_string(),
        BinaryOp::Exp => "BinaryOp::Exp".to_string(),
        BinaryOp::And => "BinaryOp::And".to_string(),
        BinaryOp::BitAnd => "BinaryOp::BitAnd".to_string(),
        BinaryOp::Or => "BinaryOp::Or".to_string(),
        BinaryOp::BitOr => "BinaryOp::BitOr".to_string(),
        BinaryOp::BitXor => "BinaryOp::BitXor".to_string(),
        BinaryOp::Shl => "BinaryOp::Shl".to_string(),
        BinaryOp::Shr => "BinaryOp::Shr".to_string(),
        BinaryOp::Sar => "BinaryOp::Sar".to_string(),
    }
}

fn CmpOp2Idx(op: &CmpOp) -> String {
    match op {
        CmpOp::Eq => "CmpOp::Eq".to_string(),
        CmpOp::Ne => "CmpOp::Ne".to_string(),
        CmpOp::Gt => "CmpOp::Gt".to_string(),
        CmpOp::Ge => "CmpOp::Ge".to_string(),
        CmpOp::Lt => "CmpOp::Lt".to_string(),
        CmpOp::Le => "CmpOp::Le".to_string(),
    }
}

fn InstrDescription2Idx(instr: &Instr) -> String {
    match &instr.inner {
        smart_ir::ir::cfg::InstrDescription::Declaration { id, init_val, ty } => {
            "InstrDescription::Declaration".to_string()
        }
        smart_ir::ir::cfg::InstrDescription::Assignment { id, val } => {
            "InstrDescription::Assignment".to_string()
        }
        smart_ir::ir::cfg::InstrDescription::Ret { val } => "InstrDescription::Ret".to_string(),
        smart_ir::ir::cfg::InstrDescription::Br { target } => "InstrDescription::Br".to_string(),
        smart_ir::ir::cfg::InstrDescription::BrIf {
            cond,
            then_bb,
            else_bb,
        } => "InstrDescription::BrIf".to_string(),
        smart_ir::ir::cfg::InstrDescription::Match {
            val,
            otherwise,
            jump_table,
        } => "InstrDescription::Match".to_string(),
        smart_ir::ir::cfg::InstrDescription::Not { op } => "InstrDescription::Not".to_string(),
        smart_ir::ir::cfg::InstrDescription::BitNot { op } => {
            "InstrDescription::BitNot".to_string()
        }
        smart_ir::ir::cfg::InstrDescription::Binary { op_code, op1, op2 } => {
            "InstrDescription::Binary".to_string()
        }
        smart_ir::ir::cfg::InstrDescription::Cmp { op_code, op1, op2 } => {
            "InstrDescription::Cmp".to_string()
        }
        smart_ir::ir::cfg::InstrDescription::Alloca { ty } => {
            "InstrDescription::Alloca".to_string()
        }
        smart_ir::ir::cfg::InstrDescription::Malloc { ty } => {
            "InstrDescription::Malloc".to_string()
        }
        smart_ir::ir::cfg::InstrDescription::Free { ptr } => "InstrDescription::Free".to_string(),
        smart_ir::ir::cfg::InstrDescription::GetField {
            ptr,
            field_path,
            field_ty,
        } => "InstrDescription::GetField".to_string(),
        smart_ir::ir::cfg::InstrDescription::SetField {
            ptr,
            val,
            field_path,
        } => "InstrDescription::SetField".to_string(),
        smart_ir::ir::cfg::InstrDescription::GetStoragePath { storage_path } => {
            "InstrDescription::GetStoragePath".to_string()
        }
        smart_ir::ir::cfg::InstrDescription::StorageLoad {
            storage_path,
            load_ty,
        } => "InstrDescription::StorageLoad".to_string(),
        smart_ir::ir::cfg::InstrDescription::StorageStore {
            storage_path,
            store_val,
        } => "InstrDescription::StorageStore".to_string(),
        smart_ir::ir::cfg::InstrDescription::Call {
            func_name,
            args,
            ret_ty,
        } => "InstrDescription::Call".to_string(),
        smart_ir::ir::cfg::InstrDescription::IntCast { val, target_ty } => {
            "InstrDescription::IntCast".to_string()
        }
    }
}
