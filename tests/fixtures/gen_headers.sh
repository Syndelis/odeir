#!/usr/bin/env sh
sources=$(ls *.json)
for file in "$sources"; do
    stripped="$(basename $file .json)"
    xxd -n "fixtures_$file" -i "$file" > "$stripped.h"
done
