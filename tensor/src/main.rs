mod cfg;
mod instr;

fn main() {}

#[cfg(test)]
mod parser_test {
    use smart_ir::ir::{
        builder::BasicBlockId,
        context::IRContext,
        frontend::{parser::compile, translate::translate_main_module},
        printer::IRPrinter,
    };

    use crate::{
        cfg::get_bb_graph,
        instr::{get_cfg_feats, NodeFeat},
    };
    use serde::{Deserialize, Serialize};
    use std::io::Write;
    use std::{collections::HashMap, fs::File};

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    struct Data {
        func_name: String,
        doc: String,
        cfg_feats: Vec<NodeFeat>,
        cfg_edges: HashMap<BasicBlockId, Vec<BasicBlockId>>,
    }

    use std::io::prelude::*;

    #[test]
    fn tensor_test() {
        let mut file =
            File::open("erc20.ir").expect("Failed to open file");

        let mut content = String::new();

        file.read_to_string(&mut content)
            .expect("Failed to read file");

        let module = compile(&content);
        println!("source code:");
        println!("{}", content);
        let mut ctx = IRContext::default();
        translate_main_module(&mut ctx, &module);
        let mut p = IRPrinter::new(&ctx);
        let mut w = String::new();
        p.print_main_module(&mut w).unwrap();
        println!("IR");
        println!("{}", w);
        let modules = ctx.modules.borrow();
        let module = modules.get(&ctx.main_module).unwrap();
        let contract = module.contract.clone().unwrap();
        let mut datas = vec![];
        for (name, func) in contract.functions {
            let cfg = &func.cfg;
            let cfg_edges = get_bb_graph(cfg);
            let mut cfg_feats = get_cfg_feats(cfg);
            cfg_feats.sort_by(|a, b| a.idx_in_parent.cmp(&b.idx_in_parent));
            let data = Data {
                func_name: name.clone(),
                doc: "".to_string(),
                cfg_edges,
                cfg_feats,
            };
            datas.push(data);
        }

        let json = serde_json::to_string(&datas).unwrap();
        let mut file = File::create("data.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
