#! /bin/env bash

files=$(find ../tests/c -type f -name "*.c" -exec grep -l "SWT_MODULE_CLONE_SKIP_FAILING_TEST" {} \; | xargs -n 1 basename)

# echo $files
for file in $files; do
    # echo "### $file" >> README.md
    # echo "" >> README.md
    # echo $file
    touch $file.md
done
