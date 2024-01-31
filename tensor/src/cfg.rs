use std::collections::HashMap;

use smart_ir::ir::{cfg::ControlFlowGraph, builder::BasicBlockId};

pub fn get_bb_graph(cfg: &ControlFlowGraph) -> HashMap<BasicBlockId, Vec<BasicBlockId>>{
    let mut res = HashMap::new();
    for (bbid, bb) in &cfg.basic_blocks{
        for instr in &bb.instrs{
            match &instr.inner{
                smart_ir::ir::cfg::InstrDescription::Br { target } => {
                    res.entry(*bbid).or_insert(Vec::new()).push(*target);
                },
                smart_ir::ir::cfg::InstrDescription::BrIf { cond, then_bb, else_bb } => {
                    res.entry(*bbid).or_insert(Vec::new()).push(*then_bb);
                    res.entry(*bbid).or_insert(Vec::new()).push(*else_bb);
                },
                _ => {}
            }
        }
    }
    res
}