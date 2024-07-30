#!/bin/bash

# Usage: bash test.sh swt && bash ./test.sh hw

set -eu

export YKB_TRACER=$1
export RUST_BACKTRACE=full
export YKD_SERIALISE_COMPILATION=1
export YKD_NEW_CODEGEN=1


# cargo test ::switch_nested_guard.newcg.c -- --nocapture |& tee ./switch_nested_guard.newcg.c.txt
# cargo test ::switch_many_guards_failing.newcg.c -- --nocapture |& tee ./switch_many_guards_failing.newcg.c.txt
# cargo test ::truncate.newcg.c |& tee ./truncate.newcg.c.txt

GIT_COMMIT=$(git rev-parse HEAD)
DIST="logs/${GIT_COMMIT}"

mkdir -p ${DIST}

cargo build 

# cargo test ::truncate.newcg.c -- --nocapture |& tee ./${DIST}/truncate.newcg.c.${YKB_TRACER}
# cargo test ::switch_many_guards_failing.newcg.c -- --nocapture |& tee ./${DIST}/switch_many_guards_failing.newcg.c.${YKB_TRACER}
cargo test ::switch_nested_guard.newcg.c -- --nocapture |& tee ./${DIST}/switch_nested_guard.newcg.c.${YKB_TRACER}
# cargo test ::srem.newcg.c -- --nocapture |& tee ./${DIST}/srem.newcg.c.${YKB_TRACER}
# 

