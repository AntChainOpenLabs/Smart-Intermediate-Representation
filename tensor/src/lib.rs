use std::collections::HashMap;

use cfg::get_bb_graph;
use instr::{get_cfg_feats, NodeFeat};
use serde::{Deserialize, Serialize};
use smart_ir::ir::{builder::BasicBlockId, context::IRContext};

mod cfg;
mod instr;
pub mod sol2tensor;
mod tensor;

