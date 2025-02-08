#!/bin/bash
/opt/shrinkray/bin/shrinkray --timeout 10 --parallelism 50 --no-clang-delta shrinkray_test_failed.sh ./tests/c/arithmetic.c