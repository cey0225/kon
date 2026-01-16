# kon_macros

**kon_macros** contains the procedural macros for the Kon Engine ecosystem.

## Scope

- **#[component]**: Auto-derives Debug, Clone, PartialEq for component structs.
- **#[system]**: Validates system function signatures at compile time.

## Usage

This crate is primarily designed to work alongside **kon_ecs**. While it can be installed independently, it is best used as part of the full engine suite.
```bash
# Recommended: Install the full engine
cargo add kon-engine

# Or use with kon_ecs
cargo add kon_ecs kon_macros
```

## License

MIT OR Apache-2.0
