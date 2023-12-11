// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0
pub const MAX_ADDRESS_LENGTH: usize = 28;
pub const DEFAULT_ADDRESS_LENGTH: usize = MAX_ADDRESS_LENGTH;

/// The compile target.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Target {
    /// Generate a generic object file for linking.
    Generic,
    /// Generate a WASM module file for linking.
    Wasm,
}

/// Defines the optimization level used to compile a `Module`.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OptimizationLevel {
    /// Optimization level: O0
    None = 0,
    /// Optimization level: O1
    Less = 1,
    /// Optimization level: O2
    Default = 2,
    /// Optimization level: O3
    Aggressive = 3,
}

impl OptimizationLevel {
    /// Get the optimization level string.
    /// - OptimizationLevel::None => "O0",
    /// - OptimizationLevel::Less => "O1",
    /// - OptimizationLevel::Default => "O2",
    /// - OptimizationLevel::Aggressive => "O3",
    pub fn level_string(&self) -> &'static str {
        match self {
            OptimizationLevel::None => "O0",
            OptimizationLevel::Less => "O1",
            OptimizationLevel::Default => "O2",
            OptimizationLevel::Aggressive => "O3",
        }
    }
}

impl From<OptimizationLevel> for inkwell::OptimizationLevel {
    fn from(value: OptimizationLevel) -> Self {
        match value {
            OptimizationLevel::None => inkwell::OptimizationLevel::None,
            OptimizationLevel::Less => inkwell::OptimizationLevel::Less,
            OptimizationLevel::Default => inkwell::OptimizationLevel::Default,
            OptimizationLevel::Aggressive => inkwell::OptimizationLevel::Aggressive,
        }
    }
}

/// Compile options.
#[derive(Clone)]
pub struct IROptions {
    /// Compile target including native so lib, wasm module, etc.
    pub target: Target,
    /// Whether to enable overflow checking for addition,
    /// subtraction, multiplication and division operations.
    pub overflow_check: bool,
    /// Address number of bits. The number of bits of the Address
    /// address, the default address length is 28.
    pub address_length: usize,
    /// Compiled optimization level, default is O3.
    pub opt_level: OptimizationLevel,
    /// Whether to generate wasm without contract hostapis and export abi internal functions.
    pub no_contract: bool,
    /// Disable function  inlining
    pub no_inline: bool,
    /// Print test information verbosely
    pub verbose: bool,
    /// Use llvm toolchain for more info. optional.
    pub use_llvm_toolchain: bool,
    /// Run the unit test
    pub is_test: bool,
    /// Generate coverage info
    pub coverage: bool,
    /// Create empty _start function for compatible(eg. wasi). this may slow down the perf a little.
    pub create_empty_start: bool,
}

impl Default for IROptions {
    fn default() -> Self {
        let coverage = cfg!(ir_coverage) || cfg!(test);
        Self {
            target: Target::Wasm,
            overflow_check: false,
            address_length: DEFAULT_ADDRESS_LENGTH,
            opt_level: OptimizationLevel::Aggressive,
            no_contract: false,
            no_inline: false,
            verbose: false,
            use_llvm_toolchain: false,
            is_test: false,
            coverage,
            create_empty_start: false,
        }
    }
}

impl IROptions {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        target: Target,
        overflow_check: bool,
        address_length: usize,
        opt_level: OptimizationLevel,
        no_contract: bool,
        no_inline: bool,
        verbose: bool,
        use_llvm_toolchain: bool,
        is_test: bool,
        coverage: bool,
        create_empty_start: bool,
    ) -> Self {
        Self {
            target,
            overflow_check,
            address_length,
            opt_level,
            no_contract,
            no_inline,
            verbose,
            use_llvm_toolchain,
            is_test,
            coverage,
            create_empty_start,
        }
    }
}
