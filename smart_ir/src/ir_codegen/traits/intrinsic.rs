// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use super::BackendTypes;

/// Intrinsic runtime methods
pub trait IntrinsicMethods: BackendTypes {
    /// Assert failure with a message at runtime.
    fn assert_failure(&self, msg: &str);
}
