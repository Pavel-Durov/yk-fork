#! /bin/env bash

files=$(find ../tests/c -type f -name "*.c" -exec grep -l "SWT_MODULE_CLONE_SKIP_FAILING_TEST" {} \; | xargs -n 1 basename)

# echo $files
for file in $files; do
    echo $file
    if [ -f $file.md ]; then
        echo "file $file.md already exists"
    else
        touch $file.md
    fi
    # create symlink
    echo "ln -s $file ../tests/c/$file"
    ln -s ../tests/c/$file $file
done

