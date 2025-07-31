#!/usr/bin/env bash

# Usage: ./compare_apis.sh file1.json file2.json

file1="$1"
file2="$2"

# Extract and sort apis into key-value maps: hex_id -> version
declare -A apis1
declare -A apis2

# Read APIs from file1
while IFS=$'\t' read -r hex version; do
  apis1["$hex"]="$version"
done < <(jq -r '.apis[] | @tsv' "$file1")

# Read APIs from file2
while IFS=$'\t' read -r hex version; do
  apis2["$hex"]="$version"
done < <(jq -r '.apis[] | @tsv' "$file2")

# Compare
echo "Comparing API versions..."
exit_code=0
for hex in "${!apis1[@]}"; do
  if [[ -n "${apis2[$hex]}" ]]; then
    if [[ "${apis1[$hex]}" -ne "${apis2[$hex]}" ]]; then
      echo "❌ API $hex has different versions: ${apis1[$hex]} vs ${apis2[$hex]}"
      exit_code=1
    fi
  fi
done

if [[ "$exit_code" -eq 0 ]]; then
  echo "✅ All matching APIs have the same versions."
fi

exit "$exit_code"