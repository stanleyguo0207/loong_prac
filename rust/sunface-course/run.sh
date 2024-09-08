#!/bin/bash

usage() {
  cat <<EOF

Usage: bash $0 -e <example>
    See all available examples in Cargo.toml. [[example]]

E.g.
    bash $0 -e hello

EOF
  exit 1
}

while getopts ":e:" arg; do
  case $arg in
  e)
    example=${OPTARG}
    ;;
  \?)
    echo "Invalid parameter     -${OPTARG}"
    usage
    ;;
  esac
done

if [[ ! -e $example ]];then
  echo "target is empty."
  usage
fi

cargo clippy
cargo run --example $example
