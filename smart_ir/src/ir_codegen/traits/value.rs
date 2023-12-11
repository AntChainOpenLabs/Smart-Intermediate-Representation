// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use super::BackendTypes;

/// ValueMethods defines all value APIs.
pub trait ValueMethods: BackendTypes {
    /// Construct a 64-bit int value using i64
    fn int_value(&self, v: i64) -> Self::Value;
    /// Construct a 64-bit float value using f64
    fn float_value(&self, v: f64) -> Self::Value;
    /// Construct a string value using &str
    fn string_value(&self, v: &str) -> Self::Value;
    /// Construct a bool value
    fn bool_value(&self, v: bool) -> Self::Value;
    /// Construct a None value
    fn none_value(&self) -> Self::Value;
    /// Construct a Undefined value
    fn undefined_value(&self) -> Self::Value;
    /// Construct a empty list value
    fn list_value(&self) -> Self::Value;
    /// Construct a list value with `n` elements
    fn list_values(&self, values: &[Self::Value]) -> Self::Value;
    /// Construct a empty dict value.
    fn dict_value(&self) -> Self::Value;
    /// Construct a unit value.
    fn unit_value(&self, v: f64, raw: i64, unit: &str) -> Self::Value;
    /// Construct a function value using a native function value.
    fn function_value(&self, function: Self::Function) -> Self::Value;
    /// Construct a closure function value with the closure variable.
    fn closure_value(&self, function: Self::Function, closure: Self::Value) -> Self::Value;
    /// Construct a structure function value using a native function.
    fn struct_function_value(
        &self,
        function: Self::Function,
        check_function: Self::Function,
        runtime_type: &str,
    ) -> Self::Value;
    /// Construct a builtin function value using the function name.
    fn builtin_function_value(&self, function_name: &str) -> Self::Value;
    /// Get a global value pointer named `name`.
    fn global_value_ptr(&self, name: &str) -> Self::Value;
    /// Get the global runtime context pointer.
    fn global_ctx_ptr(&self) -> Self::Value;
}

/// DerivedValueCalculationMethods defines all value base calculation APIs.
pub trait ValueCalculationMethods: BackendTypes {
    /// lhs + rhs
    fn add(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs - rhs
    fn sub(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs * rhs
    fn mul(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs / rhs
    fn div(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs % rhs
    fn r#mod(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs << rhs
    fn bit_lshift(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs >> rhs
    fn bit_rshift(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs & rhs
    fn bit_and(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs | rhs
    fn bit_or(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs ^ rhs
    fn bit_xor(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs && rhs
    fn logic_and(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs || rhs
    fn logic_or(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs == rhs
    fn cmp_equal_to(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs != rhs
    fn cmp_not_equal_to(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs > rhs
    fn cmp_greater_than(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs >= rhs
    fn cmp_greater_than_or_equal(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs < rhs
    fn cmp_less_than(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    /// lhs <= rhs
    fn cmp_less_than_or_equal(&self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
}

/// ValueCodeGen defines all value APIs.
pub trait ValueCodeGen: ValueMethods + ValueCalculationMethods {}
