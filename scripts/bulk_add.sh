#! /bin/bash

# helper script to bulk add new instructions to the db
INPUT="$1"
V6="$2"


while IFS= read -r IX; do


    if [[ "$V6" == "yes" ]]; then
        ./sighashdb-cli calculate --input "$IX" --v6
    else
        ./sighashdb-cli calculate --input "$IX"
    fi

    
done < "$INPUT"