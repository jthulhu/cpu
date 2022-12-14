#!/usr/bin/env bash

shopt -s nullglob

echecs=()
echo -n "Running asj tests: "
cd "$(dirname "$0")" || exit 2
for f in tests/*.asj
do
	../../out/asj -o result "$f" || continue
	if cmp --quiet \
	       <(python printer.py result 2>/dev/null | sed -E 's/[0-9]+ \| ([01]+) \| .*/\1/') \
	       "tests/$(basename "$f" ".asj").out"; then
		echo -n '.'
	else
		echo -n 'X'
		echecs+=("Echec sur '$(basename "$f" ".asj")'.")
	fi
done
rm -f result

echo
(( ${#echecs} != 0 )) && echo
for msg in "${echecs[@]}"; do
	echo "$msg"
done
(( ${#echecs} != 0 )) && exit 1 || exit 0
