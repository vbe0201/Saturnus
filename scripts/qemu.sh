#!/usr/bin/env bash

# This script is invoked, with the file to execute as the first argument,
# by cargo when executing `cargo run` or `cargo test` and is responsible
# for running the provided binary file in QEMU, and exiting with a correct 
# exit code so that `cargo test` produces correct results.

if [ -z "$1" ]
then
  echo "Usage: $0 <file to run>"
  exit 1
fi

echo "This script needs to be fixed to actually call QEMU!"
exit 1
