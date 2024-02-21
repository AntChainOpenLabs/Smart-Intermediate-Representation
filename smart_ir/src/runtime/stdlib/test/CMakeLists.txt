# Copyright 2013 <chaishushan{AT}gmail.com>. All rights reserved. Use of this
# source code is governed by a BSD-style license that can be found in the
# LICENSE file.

cmake_minimum_required(VERSION 3.1)
project(TEST)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_C_FLAGS
    "${CMAKE_C_FLAGS} -std=c99 \
    -Xclang \
    -fexperimental-max-bitint-width=256 \
    -Wno-int-conversion \
    -Wno-pointer-sign \
    -Wno-int-to-pointer-cast \
    -Wno-extra-tokens \
    -Wno-writable-strings \
    -Wno-deprecated-declarations \
    -Wno-incompatible-library-redeclaration \
    -Wno-implicit-function-declaration \
    -Wno-pragma-once-outside-header \
    -Wno-pointer-to-int-cast")

set(CMAKE_CXX_FLAGS
    "${CMAKE_CXX_FLAGS}\
    -Xclang \
    -fexperimental-max-bitint-width=256 \
    -Wno-int-conversion \
    -Wno-pointer-sign \
    -Wno-int-to-pointer-cast \
    -Wno-extra-tokens \
    -Wno-writable-strings \
    -Wno-deprecated-declarations \
    -Wno-incompatible-library-redeclaration \
    -Wno-implicit-function-declaration \
    -Wno-pragma-once-outside-header \
    -Wno-pointer-to-int-cast")

add_definitions(-w)
add_definitions(-DCC_LIB_TEST_MOCK)

add_library(
  cc-test-lib
  ir_c_mock.cc
  ../base64.c
  ../hex.c
  ../call_log.c
  ../stream.c
  ../rlp.c
  ../data_stream_builtin.c
  ../data_stream.c
  ../json.c
  ../mycrypto.c
  ../math.c
  ../qhash.c
  ../qhashtbl.c
  ../qstring.c
  ../qvector.c
  ../stdlib.c
  ../wasmheap.c
  ../ir_type.c
  ../ssz.c
  ../chain.c
  test.cc)

set(CCTestSources
    qhashtbl_test.cc
    qstring_test.cc
    qvector_test.cc
    math_test.cc
    base64_test.cc
    hex_test.cc
    ir_type_test.cc
    rlp_test.cc
    ssz-c/ssz_test.cc
    ssz-c/ssz_contract_test.cc
    ssz-c/ssz_contract_artifact_test.cc
    ssz-c/ssz_account_test.cc)

add_executable(cc-test ${CCTestSources} test_main.cc)

target_link_libraries(cc-test cc-test-lib)

include_directories(.)
