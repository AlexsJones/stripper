# Stripper

Stripper is a simple tool for removing words out of files.
For example "Lorem ipsum dolor sit amet" in `./test/asset/1` becomes "ipsum dolor sit amet"

It is recommended to use the `-p` flag e.g. `stripper -p Lorem` to prompt removal.

```
❯ stripper -p Cargo
Stripping out Cargo
Are you sure you want to strip out Cargo from ./target/package/st-0.1.0/Cargo.toml? [y/n]
n
Are you sure you want to strip out Cargo from ./target/package/st-0.1.0/Cargo.lock? [y/n]
n
```