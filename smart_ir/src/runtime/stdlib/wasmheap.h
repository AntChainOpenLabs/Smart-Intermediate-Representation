// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#pragma once
#ifndef WASMHEAP_H
#define WASMHEAP_H

#ifdef __cplusplus
extern "C" {
#endif

extern unsigned long
__builtin_wasm_memory_grow(int a, unsigned long pages);

#ifdef __cplusplus
} // end extern "C"
#endif

#endif