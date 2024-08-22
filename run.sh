#!/usr/bin/env bash

set -e

cargo build
socat -dd pty,raw,echo=0,link=/tmp/driver_in pty,raw,echo=0,link=/tmp/emulator_out &
socat -dd pty,raw,echo=0,link=/tmp/emulator_in pty,raw,echo=0,link=/tmp/driver_out &
socat -dd pty,raw,echo=0,link=/tmp/command_in pty,raw,echo=0,link=/tmp/command_out &
cargo run --bin emulator &
cargo run --bin driver

wait
