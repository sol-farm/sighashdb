#! /bin/bash

# helper script to bulk add new instructions to the db
INPUT="$1"


while IFS= read -r IX; do
    ./sighashdb-cli calculate "$IX"
done < "$INPUT"