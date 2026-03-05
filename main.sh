#!/usr/bin/env bash
set -uo pipefail
flag() {
	for f in "$@"
		do if ! [[ -e ".flags/$f" ]] 
			then return 1
		fi
	done
}
dirs=(
	logs
)
rm -rf bin "${dirs[@]}"
mkdir -p "${dirs[@]}"
cmd=(
	'+nightly fmt'
	clippy
	# check
	# clean
	build
)
for i in "${cmd[@]}"; do
	read -r -a arr <<< "$i"
	cargo "${arr[@]}" &> "logs/${arr[-1]}.log"
done
ln -s target/debug/dragons bin
find logs -empty -delete
