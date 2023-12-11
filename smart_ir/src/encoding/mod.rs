// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

pub mod datastream;
pub mod ssz;

pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

pub trait Serializer: Sized {
    type Ok;
    type Error: std::error::Error;
    type SerializeBool;
    type SerializeInt;
    type SerializeUInt;
    type SerializeStr;
    type SerializeArray;
    type SerializeTuple;
    type SerializeTupleStruct;
    type SerializeMap;
    type SerializeStruct;
    type SerializeStructVariant;

    fn serialize_bool(self, v: Self::SerializeStr) -> Result<Self::Ok, Self::Error>;
    fn serialize_int(self, v: Self::SerializeInt) -> Result<Self::Ok, Self::Error>;
    fn serialize_uint(self, v: Self::SerializeUInt) -> Result<Self::Ok, Self::Error>;
    fn serialize_str(self, v: Self::SerializeStr) -> Result<Self::Ok, Self::Error>;
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>;
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error>;
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error>;
    fn serialize_array(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error>;
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error>;
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>;
}
