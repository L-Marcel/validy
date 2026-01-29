#!/bin/bash
set -e

TEST_TARGET="mod"
OUTPUT_DIR="temp"

echo "Cleaning temp directory..."
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

find tests -type f -name "*.rs" | sort | while read -r file; do
  if [[ "$(basename "$file")" == "mod.rs" ]]; then
    continue
  fi

  rel_path="${file#tests/}"
  output_file="$OUTPUT_DIR/$rel_path"

  mkdir -p "$(dirname "$output_file")"

  module_path="${rel_path%.rs}"
  module_path="${module_path//\//::}"

  echo "Expanding: $module_path -> $output_file"
  cargo expand --all-features --test "$TEST_TARGET" "$module_path" > "$output_file" || true
done

echo "Done."
