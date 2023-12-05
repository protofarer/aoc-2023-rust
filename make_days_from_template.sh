#!/bin/bash

# Start and end day numbers
start_day=1
end_day=25

# Template file
template_file="template.rs"

for day in $(seq -f "%02g" $start_day $end_day)
do
    new_file="day${day}.rs"
    cp "$template_file" "src/$new_file"
    echo "Created src/$new_file"
done
