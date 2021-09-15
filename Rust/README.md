# Table of contents
- [Table of contents](#table-of-contents)
- [Reference](#reference)
- [License](#license)
- [Traits](#traits)
  - [Rust's Built-in Traits, the When, How & Why](#rusts-built-in-traits-the-when-how--why)
- [VSCode](#vscode)
  - [Extension](#extension)
  - [Debugging tests](#debugging-tests)
- [Prelude](#prelude)
  - [Result](#result)
  - [User agent name](#user-agent-name)
  - [Common derive](#common-derive)

# Reference
- [Cheats.rs](https://cheats.rs/)
  - Easy to read and find cheat sheet for Rust

# License

These are some tools to help with the licenses of the libraries the project:
- [cargo-license](https://crates.io/crates/cargo-license)
  - List licenses of the libraries
- [cargo-lichking](https://crates.io/crates/cargo-lichking)
  - Check the compatibility of your project license and the libraries licenses
- [cargo-about](https://crates.io/crates/cargo-about)
  - Cargo plugin for generating a listing of all of the crates used by a root crate, and the terms under which they are licensed.

# Traits

## Rust's Built-in Traits, the When, How & Why

Explanation about the built-in traits, nice reminder to review from time to time.

Some of the information is outdated, read with care!

https://llogiq.github.io/2015/07/30/traits.html

# VSCode

## Extension

- [Rust-in-peace Extension Pack](https://marketplace.visualstudio.com/items?itemName=gilescope.rust-in-peace)

  This will bring all the required extensions for most use cases.
    - For large projects, set `-j4` (or less) for `cargo check`. This will avoid freezing your system.

- [Rusty One Dark](https://marketplace.visualstudio.com/items?itemName=Jeraldson.vscode-rusty-onedark)

  Optimized dark theme

- [Cargo Toml snippets](https://marketplace.visualstudio.com/items?itemName=kevinkassimo.cargo-toml-snippets)

  Toml snippets

## Debugging tests

There are cases where the extensions don't work well with the tests, testing with features and workspaces are some of the cases.

For this kind of situation a configuration can be added to `launch.json`.

Replace the `<libname>` or remove the filter section for simple cases.

Features may be added to the `args` section.

For debugging only some specific tests, add the match-filter to the last `args` section.

```json
{
    "type": "lldb",
    "request": "launch",
    "name": "Debug specific test",
    "cargo": {
        "args": [
            "test",
            "--no-run",
        ],
        "filter": {
            "name": "<libname>",
            "kind": "lib"
        }
    },
    "args": ["<test filter>"],
    "cwd": "${workspaceFolder}"
},
```

Reference: https://github.com/vadimcn/vscode-lldb/issues/35

# Prelude

Here are some starting structures I like to use in a new project.

## Result

I like to have a local `Result` type, it's less to write and easier to refactor.

The ideal is to have also your own local `Error` type but I usually start with a boxed error and refactor later.

```rust
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
```

## User agent name

Good to set when making network or database queries:
```rust
pub const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
```

## Common derive

These are some derive types that I need more often then not:
```rust
#[derive(Debug, Clone, PartialEq)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
```
