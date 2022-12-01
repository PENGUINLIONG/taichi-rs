#!/bin/bash
if [ -z $TAICHI_REPO_DIR ]; then
    echo "TAICHI_REPO_DIR is not set"
    exit -1
fi

cp "scripts/generate_rust_language_binding.py" "$TAICHI_REPO_DIR/misc/generate_rust_language_binding.py"
pushd $TAICHI_REPO_DIR
python ./misc/generate_rust_language_binding.py
popd
cp $TAICHI_REPO_DIR/c_api/rust/*.rs "taichi-sys/src"
