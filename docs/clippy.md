Some useful clippy lints, can be added to root `Cargo.toml` for better developer experience

```toml
# Some lints that can be useful but should not be enabled in CI
# because of false positives and noise in tests
as_conversions = { level = "warn", priority = 1 }
cast_possible_truncation = { level = "warn", priority = 1 }
cognitive_complexity = { level = "warn", priority = 1 }
derive_partial_eq_without_eq = { level = "warn", priority = 1 }
else_if_without_else = { level = "warn", priority = 1 }
future_not_send = { level = "warn", priority = 1 }
redundant_clone = { level = "warn", priority = 1 }
unused_async = { level = "warn", priority = 1 }

# Restrictions, useful to find places where code can panic
arithmetic_side_effects = { level = "warn", priority = 1 }
expect_used = { level = "warn", priority = 1 }
float_arithmetic = { level = "warn", priority = 1 }
indexing_slicing = { level = "warn", priority = 1 }
missing_panics_doc = { level = "warn", priority = 1 }
todo = { level = "warn", priority = 1 }
unwrap_used = { level = "warn", priority = 1 }

# Find leftovers from debugging
dbg_macro = { level = "warn", priority = 1 }
print_stderr = { level = "warn", priority = 1 }
print_stdout = { level = "warn", priority = 1 }
```
