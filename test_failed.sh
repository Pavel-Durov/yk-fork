#!/bin/bash

# set -eu

export YKB_TRACER=swt

ARGS=""

if [ "$1" = "verbose" ]; then
    ARGS="-- --nocapture"
fi

export SWT_MODULE_CLONE=false

# Tests that are currently failing
echo "Running failing tests..."

# find ./tests/c -type f -name "*.c" -exec grep -l "SWT_MODULE_CLONE_SKIP_FAILING_TEST" {} \; | xargs -n 1 basename
run_test() {
    local test_name=$1
    echo "Running test: $test_name"
    export SWT_MODULE_CLONE_SKIP_FAILING_TEST=true
    export YKB_TRACER=swt
    ~/.cargo/bin/cargo test lang_tests::$test_name $ARGS
    if [ $? -eq 0 ]; then
        echo "❌ WARNING: Test $test_name unexpectedly PASSED"s
        return 1
    else
        echo "✓ Test $test_name failed as expected"
        return 0
    fi
}

# Run each test and count failures
for test in \
    ptradd.c
    arithmetic.c \
    phi3.c \
    float_store.c \
    phi1.c \
    no_trace_annotation2.c \
    simple_binop.c \
    peel1.c \
    bf.O2.c \
    simple_inline.c \
    bf.O3.c \
    sdiv.c \
    phi2.c \
    doubleinline.c \
    udiv.c \
    outline_recursion_indirect.c \
do
    total_count=$((total_count + 1))
    if run_test "$test"; then
        failed_count=$((failed_count + 1))
    fi
done

echo "Summary: $failed_count/$total_count tests failed as expected"

# If any test passed unexpectedly, exit with error
if [ $failed_count -ne $total_count ]; then
    echo "Error: Some tests passed unexpectedly!"
    exit 1
fi