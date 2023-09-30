#!/bin/bash

# https://getconnected.southwestwifi.com/current.json

# TODO: usage

output=${1:-flight_data}
sleep_interval=${2:-10}

mkdir -p "$output"

for i in $(seq 1 100); do
    filename=$(date +%s)
    curl --silent -X GET https://getconnected.southwestwifi.com/current.json | jq > "$output/$filename.json"
    sleep "$sleep_interval"
done

