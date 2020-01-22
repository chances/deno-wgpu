# Caveats

## Linux (elementary OS 5.1 Hera - Built on Ubuntu 18.04.3 LTS)

To build the project, [`rusty_v8`](https://crates.io/crates/rusty_v8) requires `glib2.0`, so point cargo to it via:

- `find /usr/ -iname "*glib*.pc"`
- `PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig cargo build`

Sometimes the Rust VS Code extension can get confused about Rustup's current toolchain.

Add in VS Code's settings:

```json
"rust.rustup": {
    "toolchain": "<result of `rustup toolchain list`>"
}
```

See, this [AskUbuntu answer](https://askubuntu.com/a/1027329/177764).
