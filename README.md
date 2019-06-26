# SOHABlog

Steady blog program for your Optimistic, Humorous and Amazing posts. Written in Rust.

## UNSTABLE

This project is currently unstable, please don't use it in production.

SQL scripts in `./migrations` won't be continuously.

Any interface may change without alert.

## Build

    cargo build

An environment variable `RUSTFLAGS="-Ctarget-feature=+crt-static"` is needed for Windows.
