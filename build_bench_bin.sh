#!/bin/env bash

BIN_OUT_DIR=${HOME}/benchmarks/bin

mkdir -p $BIN_OUT_DIR

# hwt
YKB_TRACER=hwt ${HOME}/.cargo/bin/cargo build --release
TEST_OUT_DIR="${BIN_OUT_DIR}" YKB_TRACER=hwt ${HOME}/.cargo/bin/cargo test ::bf.bench.c --release 2> /dev/null || true
echo "Moving ${BIN_OUT_DIR}/bf.bench to ${BIN_OUT_DIR}/bf.hwt"

ls -la ${BIN_OUT_DIR}

mv "${BIN_OUT_DIR}/bf.bench" "${BIN_OUT_DIR}/bf.hwt"

# swt
YKB_TRACER=swt ${HOME}/.cargo/bin/cargo build --release
TEST_OUT_DIR="${BIN_OUT_DIR}" YKB_TRACER=swt ${HOME}/.cargo/bin/cargo test ::bf.bench.c --release 2> /dev/null || true

ls -la ${BIN_OUT_DIR}

echo "Moving ${BIN_OUT_DIR}/bf.bench to ${BIN_OUT_DIR}/bf.swt.multi"
cp "${BIN_OUT_DIR}/bf.bench" "${BIN_OUT_DIR}/bf.swt.multi"
echo "Moving ${BIN_OUT_DIR}/bf.bench to ${BIN_OUT_DIR}/bf.swt.multi.patch"
mv "${BIN_OUT_DIR}/bf.bench" "${BIN_OUT_DIR}/bf.swt.multi.patch"

# run the benchmarks
${BIN_OUT_DIR}/bf.swt.multi ./tests/bf_examples/bench2.bf 
${BIN_OUT_DIR}/bf.swt.multi.patch ./tests/bf_examples/bench2.bf 
${BIN_OUT_DIR}/bf.hwt ./tests/bf_examples/bench2.bf 
