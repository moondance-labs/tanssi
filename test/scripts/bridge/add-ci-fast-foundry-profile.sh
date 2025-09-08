#!/usr/bin/env sh

foundry_toml="foundry.toml"

if [ ! -f "$foundry_toml" ]; then
  echo "⚠️  $foundry_toml not found in $(pwd). Skipping."
  exit 0
fi

if grep -q "^\[profile\.ci-fast\]" "$foundry_toml"; then
  echo "ℹ️  [profile.ci-fast] already exists. Skipping."
  exit 0
fi

cat >> "$foundry_toml" <<'EOF'

[profile.ci-fast]
via_ir = false
optimizer = true
EOF

echo "✅ [profile.ci-fast] added to $foundry_toml"