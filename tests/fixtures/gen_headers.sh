#!/usr/bin/env sh
cd ..
sources=$(ls fixtures/*.json)
for file in "$sources"; do
    stripped="$(basename $file .json)"
    xxd -i "$file" > "fixtures/$stripped.h"
done
