#!/bin/sh

# Add warning for undefined varables
set -u
# Add warning for unset variables
set -o nounset
# Add warning for errors in commands
set -o errexit
# Add warning for errors in pipelines
set -o pipefail
# Add warning for errors in commands
set -o errtrace

results_file="sim_results.json"
# Check if the results file already exists and remove it
if [ -f "$results_file" ]; then
    echo "Removing existing results file: $results_file"
    rm "$results_file"
fi

program="../target/debug/gridcover"
if [ ! -f "$program" ]; then
    echo "Error: $program not found. Please build the project first."
    exit 1
fi

# Create an array with strings that are program command line arguments to gridcover. 
# A typical parameter list might be "-w 500 -g 600  -c 98 -C false -T blade -r 0.5 -s 0.015 --perturb-segment true -S 999"
args=("-w 500 -g 600  -c 95 -C false -T blade -r 0.5 -s 0.015 --perturb-segment true -S 999 -J true -R false" 
"-w 500 -g 600  -c 92 -C false -T blade -r 0.5 -s 0.015 --perturb-segment true -S 999 -J true -P false -R false"
"-w 500 -g 600  -c 90 -C false -T blade -r 0.5 -s 0.015 --perturb-segment true -S 999 -J true -R false")

# Initialize the results.json with the start of a JSON array
echo "[" > $results_file
first_entry=true
# Loop through the array and run the program with each set of arguments
for arg in "${args[@]}"; do
    echo "Running with arguments: \"${arg}\""
    # If not the first entry add a ',' to separate JSON objects
    if [ "$first_entry" = true ]; then
        first_entry=false
    else
        # Add a comma to separate JSON objects
        echo "," >> $results_file
    fi
    $program $arg >> $results_file
    if [ $? -ne 0 ]; then
        echo "Error: Program failed with arguments: $arg"
        exit 1
    fi
done

# Close the JSON array
echo "]" >> ${results_file}

echo "All tests completed successfully. Results stored in ${results_file}"
