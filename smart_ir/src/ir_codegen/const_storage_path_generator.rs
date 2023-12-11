// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use indexmap::IndexMap;
use rand::AsByteSliceMut;
use std::cell::RefCell;

#[derive(Default)]
pub struct ConstStoragePathGenerator {
    pub const_storage_path_bytes: RefCell<Vec<u8>>,
    pub const_storage_path_objs: RefCell<IndexMap<Vec<u32>, u32>>,
}

impl ConstStoragePathGenerator {
    pub fn new() -> ConstStoragePathGenerator {
        ConstStoragePathGenerator {
            const_storage_path_bytes: RefCell::new(Vec::new()),
            const_storage_path_objs: RefCell::new(IndexMap::default()),
        }
    }

    pub fn get_const_storage_path_global_name(&self) -> String {
        "const_storage_paths".to_string()
    }

    // put all const storage path offset in the end of global data,
    // return offset and len
    pub fn finalize_const_storage_path_data(&self) -> (u32, u32) {
        let mut global_datas = self.const_storage_path_bytes.borrow_mut();
        let path_objs = self.const_storage_path_objs.borrow();

        let mut csp_list: Vec<u32> = Vec::new();
        for (_, offset) in path_objs.iter() {
            csp_list.push(*offset);
        }

        let csp_list_offset = global_datas.len() as u32;
        let csp_list_length = csp_list.len() as u32;
        global_datas.extend_from_slice(csp_list.as_byte_slice_mut());

        (csp_list_offset, csp_list_length)
    }
}
