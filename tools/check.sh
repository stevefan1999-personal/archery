#!/bin/bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

set -e

cd $(dirname "$0")
cd "$(git rev-parse --show-toplevel)"

source "tools/utils.sh"

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

function on_failure {
    echo >&2
    echo -e "${RED}Whoopsie-daisy: something failed!$NC" >&2
}

assert_installed "cargo-deadlinks"
assert_installed "cargo-fmt"
assert_installed "cargo-miri"

trap on_failure ERR

echo 'Building:'
cargo build --features fatal-warnings --all-targets
echo 'Testing:'
cargo test --features fatal-warnings --all-targets
echo 'Checking documentation:'
cargo doc --features fatal-warnings --no-deps --document-private-items

# Tests for memory safety and memory leaks with miri.
if [ -z "$MIRI_TOOLCHAIN" ]; then
    MIRI_TOOLCHAIN=nightly
fi
echo "Testing with miri (with toolchain $MIRI_TOOLCHAIN):"
cargo +$MIRI_TOOLCHAIN miri test

echo 'Checking links:'
cargo deadlinks

echo 'Checking packaging:'
cargo package --allow-dirty
echo 'Checking code style:'
cargo fmt -- --check
echo 'Checking readme:'
cargo rdme --check

echo
echo -e "${GREEN}Everything looks lovely!$NC"

exit 0
