#! /bin/bash

set -euxo pipefail

rsync_cmd bencher16.soft-dev.org YK_LOG=4 CP_TRANSITION_DEBUG_MODE=true YKB_TRACER=swt /home/pd/.cargo/bin/cargo test bf.O1 -- --nocapture |& tee ~/temp/bf.O1.log
rsync_cmd bencher16.soft-dev.org YK_LOG=4 CP_TRANSITION_DEBUG_MODE=true YKB_TRACER=swt /home/pd/.cargo/bin/cargo test bf.O0 -- --nocapture |& tee ~/temp/bf.O0.log
