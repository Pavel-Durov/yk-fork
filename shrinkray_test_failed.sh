#!/bin/bash

# Run the test and check for segfault
cd /home/pd/yk-fork
YKB_TRACER=swt ~/.cargo/bin/cargo test lang_tests::arithmetic.c

if [ $? -eq 1 ]; then
    echo "interesting!"
    exit 0
else
    echo "not interesting"
    exit 1
fi