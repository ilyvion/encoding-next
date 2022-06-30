#!/bin/bash

# set the destination for profile files
export LLVM_PROFILE_FILE="target/debug/coverage/%p-%m.profraw"

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zinstrument-coverage -Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"

# build the test binary with coverage instrumentation
executables=($(cargo test --workspace --tests --no-run --all-features --message-format=json | jq -r "select(.profile.test == true) | .executable"))

# run instrumented tests
for e in "${executables[@]}"
do
    $e
done

if [[ "${CI}" ]]
then
echo "building coverage for ci"
./grcov . \
    -s . \
    --binary-path ./target/debug/ \
    -t lcov \
    --branch \
    --ignore-not-existing \
    -o ./target/debug/coverage/lcov.info
else
echo "building coverage for local"
grcov . \
    -s . \
    --binary-path ./target/debug/ \
    -t lcov \
    --branch \
    --ignore-not-existing \
    -o ./target/debug/coverage/lcov.info

grcov . \
    -s . \
    --binary-path ./target/debug/ \
    -t html \
    --branch \
    --ignore-not-existing \
    -o ./target/debug/coverage/html
open ./target/debug/coverage/html/index.html
fi