#!/bin/bash

set -eu

export RUST_BACKTRACE=full
export YKD_SERIALISE_COMPILATION=1
export YKD_NEW_CODEGEN=1truncate.newcg.c 
export YKB_TRACER=swt

cargo test ::switch_nested_guard.newcg.c -- --nocapture |& tee ./switch_nested_guard.newcg.c.txt
cargo test ::switch_many_guards_failing.newcg.c -- --nocapture |& tee ./switch_many_guards_failing.newcg.c.txt
cargo test ::truncate.newcg.c |& tee ./truncate.newcg.c.txt
