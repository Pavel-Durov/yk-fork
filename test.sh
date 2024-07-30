#!/bin/bash
# Usage: bash test.sh swt

set -eu

export YKB_TRACER=$1
export RUST_BACKTRACE=full
export YKD_SERIALISE_COMPILATION=1
export YKD_NEW_CODEGEN=1

GIT_COMMIT=$(git rev-parse HEAD)
DIST="logs/${GIT_COMMIT}"

mkdir -p ${DIST}

cargo build 

cargo test ::switch_nested_guard.newcg.c -- --nocapture |& tee ./${DIST}/switch_nested_guard.newcg.c.${YKB_TRACER}
