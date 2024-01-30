/// Convert the json file crawled from etherscan to the standard_json input of the sol compiler,
/// standard output, yul, sir and tensor(json)
/// https://learnblockchain.cn/docs/solidity/using-the-compiler.html#json
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use smart_ir::ir::context::IRContext;
use smart_ir::ir::printer::IRPrinter;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::vec::Vec;

use crate::tensor::ir2tensor;
use crate::tensor::TensorData;

#[derive(Serialize, Deserialize, Debug, Clone)]
/// filed name get from input json, dont edit it
struct Data {
    SourceCode: String,
    OptimizationUsed: String,
    Runs: String,
    Library: String,
    CompilerVersion: String,
    ContractName: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SourceInfo {
    contract_address: String,
    contract_name: String,
    solc_version: String,
    standard_json: Value,
}

type Address = String;
type YulSrc = String;

#[derive(Debug, Default)]
pub struct DataMap {
    pub inputs: HashMap<Address, Data>,
    pub source_infos: HashMap<Address, SourceInfo>,
    pub standard_inputs: HashMap<Address, Value>,
    pub standard_output: HashMap<Address, Value>,
    pub yul: HashMap<Address, HashMap<String, YulSrc>>,
    pub sir: HashMap<Address, HashMap<String, IRContext>>,
    pub tensor: HashMap<Address, HashMap<String, Vec<TensorData>>>,
}

impl DataMap {
    fn dump_to_file(
        &self,
        source_info_dir: &str,
        standard_input_dir: &str,
        standard_output_dir: &str,
        yul_dir: &str,
        sir_dir: &str,
        tensor_dir: &str,
    ) {
        for (address, source_info) in &self.source_infos {
            let file_name = format!("{}/{}.json", source_info_dir, address);
            let file_path = Path::new(&file_name);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            let file = File::create(file_path).unwrap();
            serde_json::to_writer_pretty(file, &source_info).unwrap();
        }

        for (address, standard_input) in &self.standard_inputs {
            let file_name = format!("{}/{}.json", standard_input_dir, address);
            let file_path = Path::new(&file_name);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            let file = File::create(file_path).unwrap();
            serde_json::to_writer_pretty(file, &standard_input).unwrap();
        }

        for (address, standard_output) in &self.standard_output {
            let file_name = format!("{}/{}.json", standard_output_dir, address);
            let file_path = Path::new(&file_name);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            let file = File::create(file_path).unwrap();
            serde_json::to_writer_pretty(file, &standard_output).unwrap();
        }

        for (_, yuls) in &self.yul {
            for (fqn_name, yul_src) in yuls {
                let file_name = format!("{}/{}.yul", yul_dir, fqn_name);
                let file_path = Path::new(&file_name);
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                let mut file = File::create(file_name).unwrap();
                writeln!(file, "{}", yul_src).unwrap();
            }
        }

        for (_, ctxs) in &self.sir {
            for (fqn_name, ctx) in ctxs {
                let file_name = format!("{}/{}.yul", sir_dir, fqn_name);
                let file_path = Path::new(&file_name);
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                let mut p = IRPrinter::new(&ctx);
                let mut w = String::new();
                p.print_modules(&mut w).unwrap();
                let mut file = File::create(file_name).unwrap();
                writeln!(file, "{}", w).unwrap();
            }
        }

        for (_, datas) in &self.tensor {
            for (fqn_name, data) in datas {
                let file_name = format!("{}/{}.json", tensor_dir, fqn_name);
                let file_path = Path::new(&file_name);
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                let json = serde_json::to_string(&data).unwrap();
                let mut file = File::create(file_name).unwrap();
                file.write_all(json.as_bytes()).unwrap();
            }
        }
    }
}

fn load_sources(path: &str) -> (Vec<String>, Vec<String>, DataMap) {
    let mut data_map = DataMap::default();
    let mut success: Vec<String> = vec![];
    let mut failed: Vec<String> = vec![];
    for entry in std::fs::read_dir(path).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            if file_path.is_dir() {
                continue;
            }
            if let Some(file_name) = file_path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if let Some(source_info) = load_source(&file_path) {
                        success.push(file_name_str.to_string());
                        data_map
                            .source_infos
                            .insert(source_info.contract_address.clone(), source_info.clone());
                        data_map.standard_inputs.insert(
                            source_info.contract_address.clone(),
                            source_info.standard_json.clone(),
                        );
                    } else {
                        failed.push(file_name_str.to_string());
                    }
                }
            }
        }
    }

    (success, failed, data_map)
}

fn read_output_from_file(standard_output_dir: &str) -> HashMap<Address, Value> {
    let mut standard_output = HashMap::default();
    for entry in std::fs::read_dir(standard_output_dir).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            if let Some(file_name) = file_path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    let contract_address = file_name_str.split('.').next().unwrap().to_string();
                    let mut file = File::open(&file_path).expect("Failed to open file");
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)
                        .expect("Failed to read file");

                    let data = match serde_json::from_str(&contents) {
                        Ok(data) => data,
                        Err(_) => {
                            continue;
                        }
                    };
                    standard_output.insert(contract_address, data);
                }
            }
        }
    }
    standard_output
}

fn load_source(file_path: &PathBuf) -> Option<SourceInfo> {
    if let Some(file_name) = file_path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            let contract_address = file_name_str.split('.').next().unwrap().to_string();
            let mut file = File::open(&file_path).expect("Failed to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read file");

            let data: Data = match serde_json::from_str(&contents) {
                Ok(data) => data,
                Err(_) => {
                    return None;
                }
            };

            let solc_version = match regex::Regex::new(r"v(.*?)\+commit") {
                Ok(regex) => {
                    match regex.captures(&data.CompilerVersion) {
                        Some(captures) => captures.get(1).unwrap().as_str().to_string(),
                        None => {
                            return None;
                        }
                    }
                }
                Err(_) => {
                    return None;
                }
            };
            let standard_json = match get_sol_standard_input_json(
                &data.SourceCode,
                data.OptimizationUsed == "1",
                data.Runs.parse().unwrap(),
                &data.Library,
                json!({
                    "*": {
                        "*": ["ir", "userdoc", "devdoc"],
                        "": ["ast"],
                    },
                }),
            ) {
                Ok(standard_json) => standard_json,
                Err(_) => {
                    return None;
                }
            };

            let source_info = SourceInfo {
                contract_address: contract_address.clone(),
                contract_name: data.ContractName.clone(),
                solc_version,
                standard_json: standard_json.clone(),
            };
            return Some(source_info);
        }
    }
    None
}

fn get_sol_standard_input_json(
    code: &str,
    optimized: bool,
    optimization_runs: usize,
    libraries: &str,
    output_selection: Value,
) -> Result<Value, String> {
    // build standard json
    let _tmp_filename = "this_is_a_tmp_filename.sol";
    let new_code = code.replace("\r\n", "\n");
    let new_code = new_code.replace("{{", "{");
    let new_code = new_code.replace("}}", "}");
    let mut standard_json = match serde_json::from_str::<Value>(&new_code) {
        Ok(json) => json,
        Err(e) => {
            let mut standard_json = json!({
                "language": "Solidity",
                "settings": {
                    "optimizer": {
                        "enabled": optimized,
                        "runs": optimization_runs,
                    },
                },
                "sources": {
                    _tmp_filename: {
                        "content": code.replace("\r\n", "\n"),
                    },
                },
            });
            if !libraries.is_empty() {
                match serde_json::from_str(libraries) {
                    Ok(libraries) => standard_json["settings"]["libraries"] = libraries,
                    Err(_) => return Err("libraries not found in data".to_string()),
                }
            }
            standard_json
        }
    };
    standard_json["settings"]["outputSelection"] = output_selection;
    Ok(standard_json)
}

// compile solidity source code by solcjs, need `npm install` to install dependencies
fn compile_by_solcjs(m: &mut DataMap) {
    let mut success = vec![];
    let mut failed = vec![];
    let len = m.source_infos.len();
    for (i, (address, source_info)) in m.source_infos.iter().enumerate() {
        println!("compile: {:?}, {:?}/{:?}", address, i + 1, len);
        match _compile_by_solcjs(&source_info) {
            Some(val) => {
                success.push(address);
                m.standard_output.insert(address.clone(), val);
            }
            None => failed.push(address),
        }
    }
}

fn _compile_by_solcjs(source_info: &SourceInfo) -> Option<Value> {
    let solcjs = format!(
        "const solc = require('solcv{}');
const input = {:};
var output = solc.compile(JSON.stringify(input));
if (output instanceof Object) {{
    output = JSON.stringify(output);
}}
console.log(output)
        ",
        &source_info.solc_version,
        serde_json::to_string(&&source_info.standard_json).unwrap()
    );
    let output = Command::new("node").arg("-e").arg(solcjs).output().unwrap();
    let res = String::from_utf8(output.stdout).unwrap();
    match serde_json::from_str::<Value>(&res) {
        Ok(val) => {
            if val["errors"].as_array().is_none() {
                return Some(val);
            } else {
                return None;
            }
        }
        Err(_) => return None,
    }
}

// get yul from standard output json
fn get_yul(m: &mut DataMap) {
    for (address, val) in &m.standard_output {
        let irs = _get_yul(&address, val);
        m.yul.insert(address.clone(), irs);
    }
}

// get yul from standard output json
fn _get_yul(address: &str, address_val: &Value) -> HashMap<String, YulSrc> {
    let mut irs = HashMap::default();
    let contrats = address_val["contracts"].as_object().unwrap();
    for (file_name, files) in contrats {
        let files = files.as_object().unwrap();
        for (contract_name, contract) in files {
            match contract.get("ir") {
                Some(ir) => {
                    if let Some(ir) = ir.as_str() {
                        if ir.len() > 0 {
                            let key = format!(
                                "{}:{}:{}",
                                address,
                                file_name.split('/').last().unwrap(),
                                contract_name
                            );
                            irs.insert(key, ir.to_string());
                        }
                    }
                }
                None => {}
            }
        }
    }
    return irs;
}

fn yul2tenosr(m: &mut DataMap) {
    for (address, irs) in &m.yul {
        let (ctxs, tensors) = _yul2tensor(irs);
        m.sir.insert(address.clone(), ctxs);
        m.tensor.insert(address.clone(), tensors);
    }
}

fn _yul2tensor(
    irs: &HashMap<String, YulSrc>,
) -> (HashMap<String, IRContext>, HashMap<String, Vec<TensorData>>) {
    let mut ctxs = HashMap::default();
    let mut tensors = HashMap::default();
    for (fqn_name, yul_src) in irs {
        match yul_to_ir::yul2ir(&yul_src, None) {
            Some(ctx) => {
                let ctx = ctx.ir_context;
                ctxs.insert(fqn_name.clone(), ctx.clone());
                let datas = ir2tensor(ctx);
                tensors.insert(fqn_name.clone(), datas);
            }
            None => {}
        }
    }
    (ctxs, tensors)
}

fn get_contract_user_doc(standard_output_json_val: &Value, contract_name: String) -> Value {
    standard_output_json_val["contracts"]["this_is_a_tmp_filename.sol"][contract_name]["userdoc"]
        ["methods"]
        .clone()
}

fn get_contract_dev_doc(standard_output_json_val: &Value, contract_name: String) -> Value {
    standard_output_json_val["contracts"]["this_is_a_tmp_filename.sol"][contract_name]["devdoc"]
        ["methods"]
        .clone()
}

pub fn sol_to_tensor(input_path: &str, dump_to_file: bool) -> DataMap {
    let path = PathBuf::from(input_path);
    println!("{:?}", "load sources");
    let mut data_map = load_sources(input_path).2;
    println!("{:?}", "compiled by soljs");
    compile_by_solcjs(&mut data_map);
    println!("{:?}", "getyul");
    get_yul(&mut data_map);
    println!("{:?}", "yul to ir");
    yul2tenosr(&mut data_map);

    if dump_to_file {
        let path = path.parent().unwrap_or(&path).to_path_buf();
        let mut source_info_dir = path.clone();
        let mut standard_input_dir = path.clone();
        let mut standard_output_dir = path.clone();
        let mut yul_dir = path.clone();
        let mut sir_dir = path.clone();
        let mut tensor_dir = path.clone();
        source_info_dir.push("output/source_info");
        standard_input_dir.push("output/standard_input");
        standard_output_dir.push("output/standard_output");
        yul_dir.push("output/yul");
        sir_dir.push("output/sir");
        tensor_dir.push("output/tensor");
        println!("{:?}", "dump");
        data_map.dump_to_file(
            source_info_dir.to_str().unwrap(),
            standard_input_dir.to_str().unwrap(),
            standard_output_dir.to_str().unwrap(),
            yul_dir.to_str().unwrap(),
            sir_dir.to_str().unwrap(),
            tensor_dir.to_str().unwrap(),
        );
    }

    println!("source info {:?}", data_map.source_infos.len());
    println!("in {:?}", data_map.standard_inputs.len());
    println!("out {:?}", data_map.standard_output.len());
    let mut yul_count = 0;
    let mut sir_count = 0;
    let mut tensor_count = 0;
    for (_, yuls) in &data_map.yul {
        yul_count += yuls.len();
    }
    for (_, sirs) in &data_map.sir {
        sir_count += sirs.len();
    }
    for (_, tensors) in &data_map.tensor {
        tensor_count += tensors.len();
    }
    println!("yul: {:?}", yul_count);
    println!("sir {:?}", sir_count);
    println!("tensor {:?}", tensor_count);
    data_map
}

fn sol_to_tensor_by_file(
    file_path: &PathBuf,
) -> Option<(
    Address,
    SourceInfo,
    Value,
    HashMap<String, YulSrc>,
    HashMap<String, IRContext>,
    HashMap<String, Vec<TensorData>>,
)> {
    match load_source(file_path) {
        Some(source_info) => {
            if let Some(val) = _compile_by_solcjs(&source_info) {
                let irs = _get_yul(&source_info.contract_address, &val);
                let (ctxs, tensors) = _yul2tensor(&irs);
                return Some((
                    source_info.contract_address.clone(),
                    source_info,
                    val,
                    irs,
                    ctxs,
                    tensors,
                ));
            }
        }
        None => {}
    }
    return None;
}

#[cfg(test)]
mod standard_json_test {
    use super::sol_to_tensor;
    use std::path::PathBuf;

    #[test]
    fn sol2tensor_test() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src/test_data/input");
        sol_to_tensor(path.to_str().unwrap(), true);
    }
}
