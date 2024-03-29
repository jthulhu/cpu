#!/bin/bash

cd "$(dirname "$0")/../.." || exit 2
output="$1_$2_$3_$4_$5_$6_$7"
if [[ "$output" =~ ^[0-9]+_[0-9]+_[0-9]+_[0-9]+_[0-9]+_[0-9]+_[0-9]+$ ]] && ! [[ -e tests/program/"$output" ]]
then
	tests/program/clock_test.py "$@" >tests/program/$output
fi
