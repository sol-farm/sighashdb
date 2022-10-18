#! /bin/bash

# helper script to bulk add new instructions to the db
INPUT="$1"
V6="$2"


while IFS= read -r IX; do


    if [[ "$V6" == "yes" ]]; then
        echo "v6 mode"
        ./sighashdb-cli calculate --input "$IX" --v6
    else
        OUTPUT=$(./sighashdb-cli calculate --input "$IX")
        NAME=$(echo "$OUTPUT" | sed 2d | awk '{print $1}' | tr '[:lower:]' '[:upper:]' | tr -d '"')
        IX_DATA=$(echo "$OUTPUT" | sed 1d | awk -F "=>" '{print $1}')
        echo "pub const $NAME: [u8; 8] = $IX_DATA;"
    fi

    
done < "$INPUT"