use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use smart_ir::ir::{builder::BasicBlockId, context::IRContext};

use crate::{
    cfg::get_bb_graph,
    instr::{get_cfg_feats, NodeFeat},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TensorData {
    pub func_name: String,
    pub doc: String,
    cfg_feats: Vec<NodeFeat>,
    cfg_edges: HashMap<BasicBlockId, Vec<BasicBlockId>>,
}

pub fn ir2tensor(ctx: IRContext) -> Vec<TensorData> {
    let modules = ctx.modules.borrow();
    let mut datas = vec![];
    for (_, module) in modules.iter(){
        let contract = module.contract.clone().unwrap();
   
        for (name, func) in contract.functions {
            let cfg = &func.cfg;
            let cfg_edges = get_bb_graph(cfg);
            let mut cfg_feats = get_cfg_feats(cfg);
            cfg_feats.sort_by(|a, b| a.idx_in_parent.cmp(&b.idx_in_parent));
            let data: TensorData = TensorData {
                func_name: name.clone(),
                doc: "".to_string(),
                cfg_edges,
                cfg_feats,
            };
            datas.push(data);
        }
    }

    datas
}
