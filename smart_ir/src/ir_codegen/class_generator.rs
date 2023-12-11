// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use byteorder::{LittleEndian, WriteBytesExt};
use indexmap::IndexMap;
use std::cell::RefCell;

use crate::ir::cfg::{Type, TypeDefinitionKind};

use crate::ir::context::IRContext;

pub struct ClassGenerator<'ctx> {
    pub all_runtime_classes_bytes: RefCell<Vec<u8>>, // constant bytes to store all runtime classes
    pub type_runtime_classes: RefCell<IndexMap<Type, u32>>, // type id => offset in const all_runtime_classes_bytes
    pub runtime_const_str_objs: RefCell<IndexMap<String, u32>>,
    pub ir_context: &'ctx IRContext,
}

impl<'ctx> ClassGenerator<'ctx> {
    pub fn new(ir_ctx: &IRContext) -> ClassGenerator {
        ClassGenerator {
            all_runtime_classes_bytes: RefCell::new(Vec::new()),
            type_runtime_classes: RefCell::new(IndexMap::default()),
            runtime_const_str_objs: RefCell::new(IndexMap::default()),
            ir_context: ir_ctx,
        }
    }

    pub fn get_ir_runtime_classes_global_name(&self) -> String {
        "ty_ir_runtime_classes".to_string()
    }

    fn create_ir_runtime_const_str(&self, str_bytes_len: u32, string_bytes_offset: u32) -> Vec<u8> {
        let mut string_runtime_obj_bytes: Vec<u8> = vec![];
        string_runtime_obj_bytes
            .write_u32::<LittleEndian>(str_bytes_len)
            .unwrap();
        string_runtime_obj_bytes
            .write_u32::<LittleEndian>(str_bytes_len)
            .unwrap();
        string_runtime_obj_bytes
            .write_u32::<LittleEndian>(string_bytes_offset)
            .unwrap();
        string_runtime_obj_bytes
    }
    fn create_ir_runtime_class(&self, ty: &Type) -> Vec<u8> {
        let ty = if let Type::Pointer(deref_ty) = ty {
            deref_ty
        } else {
            ty
        };
        let mut ty_runtime_class_bytes: Vec<u8> = vec![];
        // size of value in memory
        let type_size: u32 = 0; // Currently, memory size of type object is not used, and will be automatically computed in ir_type.c
        let type_enum_in_c: u32 = self.ir_context.ir_runtime_class_c_enum(ty);
        let mut struct_fields_types_offsets: Vec<u32> = Vec::new();
        let mut struct_fields_count: u32 = 0;
        let mut struct_fields_names_offsets: Vec<u32> = Vec::new();

        let mut array_item_ty_offset: u32 = 0;
        let mut array_size: u32 = 0;
        let mut map_key_ty_offset: u32 = 0;
        let mut map_value_ty_offset: u32 = 0;
        match ty {
            Type::Def(def) => match def.kind {
                TypeDefinitionKind::Struct => {
                    if let Type::Compound(fields) = def.ty.as_ref() {
                        for field in fields.iter() {
                            struct_fields_count += 1;
                            struct_fields_types_offsets
                                .push(self.intern_ir_runtime_class(&field.ty));
                            struct_fields_names_offsets
                                .push(self.create_ir_runtime_const_str_obj(&field.name));
                        }
                    } else {
                        unimplemented!()
                    }
                }
                TypeDefinitionKind::Alias => return self.create_ir_runtime_class(&def.ty),
                _ => unimplemented!(),
            },
            Type::Array { elem, len } => {
                array_item_ty_offset = self.intern_ir_runtime_class(elem);
                if let Some(size) = len {
                    array_size = *size;
                }
            }
            Type::Map { key, value } => {
                map_key_ty_offset = self.intern_ir_runtime_class(key);
                map_value_ty_offset = self.intern_ir_runtime_class(value);
            }
            _ => (),
        };
        // struct_fields_types_offsets array put into bytes of all_runtime_classes. then we can get the memory offset of it.
        let struct_fields_types_offsets_array_offset =
            self.write_u32_array_to_type_runtime_classes(&struct_fields_types_offsets);
        let struct_fields_names_offsets_array_offset =
            self.write_u32_array_to_type_runtime_classes(&struct_fields_names_offsets);
        // write bytes
        ty_runtime_class_bytes
            .write_u32::<LittleEndian>(type_size)
            .unwrap();
        ty_runtime_class_bytes
            .write_u32::<LittleEndian>(type_enum_in_c)
            .unwrap();
        ty_runtime_class_bytes
            .write_u32::<LittleEndian>(struct_fields_types_offsets_array_offset)
            .unwrap();
        ty_runtime_class_bytes
            .write_u32::<LittleEndian>(struct_fields_count)
            .unwrap();
        ty_runtime_class_bytes
            .write_u32::<LittleEndian>(struct_fields_names_offsets_array_offset)
            .unwrap();
        ty_runtime_class_bytes
            .write_u32::<LittleEndian>(array_item_ty_offset)
            .unwrap();
        ty_runtime_class_bytes
            .write_u32::<LittleEndian>(array_size)
            .unwrap();
        ty_runtime_class_bytes
            .write_u32::<LittleEndian>(map_key_ty_offset)
            .unwrap();
        ty_runtime_class_bytes
            .write_u32::<LittleEndian>(map_value_ty_offset)
            .unwrap();
        ty_runtime_class_bytes
    }

    /// Get or create IRRuntimeClass for one IR type.
    /// This api just create IRRuntimeClass bytes and store in context
    /// need call finalize_runtime_classes to write the constant bytes to llvm ir
    pub fn intern_ir_runtime_class(&self, ty: &Type) -> u32 {
        let old_cache = { self.type_runtime_classes.borrow().get(ty).copied() };
        match old_cache {
            Some(cache) => cache,
            None => {
                let mut ty_runtime_class_bytes = self.create_ir_runtime_class(ty);
                let offset = { self.all_runtime_classes_bytes.borrow().len() as u32 };
                self.all_runtime_classes_bytes
                    .borrow_mut()
                    .append(&mut ty_runtime_class_bytes);
                self.type_runtime_classes
                    .borrow_mut()
                    .insert(ty.clone(), offset);
                offset
            }
        }
    }

    /// write [u32] to self.all_runtime_classes_bytes bytes, return the insert offset
    fn write_u32_array_to_type_runtime_classes(&self, value: &Vec<u32>) -> u32 {
        let offset = { self.all_runtime_classes_bytes.borrow().len() };
        for item in value {
            self.all_runtime_classes_bytes
                .borrow_mut()
                .write_u32::<LittleEndian>(*item)
                .unwrap();
        }
        offset as u32
    }

    fn create_ir_runtime_const_str_obj(&self, s: &str) -> u32 {
        let old_cache = { self.runtime_const_str_objs.borrow().get(s).copied() };
        match old_cache {
            Some(cache) => cache,
            None => {
                let string_bytes_offset = { self.all_runtime_classes_bytes.borrow().len() as u32 };
                let string_bytes = s.as_bytes().to_vec();
                self.all_runtime_classes_bytes
                    .borrow_mut()
                    .append(&mut string_bytes.clone());
                let str_obj_offset = { self.all_runtime_classes_bytes.borrow().len() as u32 };
                let mut string_obj_bytes = self
                    .create_ir_runtime_const_str(string_bytes.len() as u32, string_bytes_offset);

                self.all_runtime_classes_bytes
                    .borrow_mut()
                    .append(&mut string_obj_bytes);

                self.runtime_const_str_objs
                    .borrow_mut()
                    .insert(s.to_string(), str_obj_offset);
                str_obj_offset
            }
        }
    }
}
