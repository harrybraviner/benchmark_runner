#! /bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null &&pwd )"

cargo run --manifest-path ${DIR}/Cargo.toml --quiet $*
