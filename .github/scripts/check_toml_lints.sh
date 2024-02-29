#!/bin/bash

# Initialize a flag to indicate failure
failure=0

# List files tracked by git, excluding those ignored by .gitignore
git ls-files | grep '^.*/Cargo.toml$' | while read -r file; do
    # Check if the file contains the [lints]\nworkspace = true pattern
    if ! grep -Pzoq "\[lints]\nworkspace = true" "$file"; then
        # If the pattern is not found, print a message and set the failure flag
        echo "Missing [lints] workspace = true in $file"
        failure=1
    else
	:;
        # If found, print a confirmation message (optional)
        echo "[lints] workspace = true found in $file"
    fi
done

# Exit with a non-zero status if any Cargo.toml file was missing the required lines
if [ "$failure" -eq 1 ]; then
    exit 1
fi

