#!/usr/bin/env bash
set -e
echo "building ir c lib unit tests"
# need clang12. clang14 not supported i256/u256
cmake -S . -B build -DCMAKE_CXX_COMPILER=clang++ -DCMAKE_C_COMPILER=clang -DCMAKE_BUILD_TYPE=Debug
cmake --build build -j8
PWD=`pwd`
cd build

echo "-----------------------   run c lib unit tests   ---------------------"
./cc-test
echo "----------------------- c lib unit tests run done ---------------------"
cd $PWD
