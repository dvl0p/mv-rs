# 🧳 mv-rs

This project is a reimplementation of `mv` in Rust.

## Project purpose

My goal with this project is to learn how to interface Rust with syscalls via the
`libc` official Rust crate.

## Approach

My approach to moving files will first assume that the source and destination
paths reside on the same filesystem partition.
Under this assumption, the move can be achieved cleanly by manipulating
directory entries rather than data blocks.
While `libc::rename` handles this atomically, `mv-rs` will manually execute a
`libc::link` followed by a `libc::unlink` to explicitly study filesystem
primitives.
This fast-path approach requires zero data transfers or disk I/O bottlenecks.

If this fast-path fails (e.g., crossing a physical disk partition boundary),
`mv-rs` catches the error and falls back to a streaming copy mechanism using
a fixed 4KB stack buffer.
Data is pulled into RAM and pushed to the destination chunk-by-chunk, meaning
the file size dictates the exact I/O footprint.

The number of sequential read or write cycles required is calculated as:
$\lfloor\frac{f - 1}{4096}\rfloor + 1$, where $f$ is the file size in bytes.

The values associated with this calculation are presented to the user via the
`-v` flag.
