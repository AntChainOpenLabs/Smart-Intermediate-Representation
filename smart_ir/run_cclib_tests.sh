#!/usr/bin/env bash
set -e
PWD=`pwd`

cd src/runtime/stdlib/test
./run_test.sh
cd $PWD