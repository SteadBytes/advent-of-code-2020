#! /usr/bin/env bash
# TODO: Make this script better...
set -eou pipefail

daynum="$1"
daymodname=d$(printf "%02d" "$daynum")
prevdaynum="$(($1-1))"
prevdaymodname=d$(printf "%02d" "$prevdaynum")

cp d0x.rs.tpl src/d$daynum.rs

echo "pub mod d${daynum};" >> src/lib.rs
if (( "$prevdaynum" > 0 )); then
	sed -i "/\"$prevdaynum\" => $prevdaymodname::run,/a \"$daynum\" => $daymodname::run," src/main.rs
fi
rustfmt src/lib.rs src/main.rs
