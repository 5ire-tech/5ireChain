#!/bin/bash

function insert_keys {
    local validator_file=$1
    local port=$2

    # Check if the validator file exists
    if [[ ! -f $validator_file ]]; then
        echo "The file '$validator_file' does not exist."
        exit 1
    fi

    # Read from the file and execute the curl command for each line
    while IFS=, read -r type mnemonic key; do
        if [ -z "$type" ] || [ -z "$mnemonic" ] || [ -z "$key" ]; then
            echo "Error: empty fields"
            continue  
        fi

        local data='{"jsonrpc":"2.0","id":1,"method":"author_insertKey","params":["'$type'","'$mnemonic'","'$key'"]}'
        
        curl "http://localhost:$port" -H "Content-Type:application/json;charset=utf-8" -d "$data"
        
        echo ""
    done < $validator_file
}

insert_keys "validator1" 9946
insert_keys "validator2" 9947
insert_keys "validator3" 9948
insert_keys "validator4" 9949
