#!/usr/bin/env bash
set -uo pipefail
dirs=(
	logs
)
rm -rf bin "${dirs[@]}"
mkdir -p "${dirs[@]}"
cmd=(
	'+nightly fmt'
	clippy
	# clean
	build
)
for i in "${cmd[@]}"; do
	read -r -a arr <<< "$i"
	cargo "${arr[@]}" &> "logs/${arr[-1]}.log"
done
ln -s target/debug/dragons bin
find logs -empty -delete
