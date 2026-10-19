# CLI File Organizer

A Rust learning project. The goal is to build a command-line file organizer, and along the way each module practices one Rust topic: file system I/O, closures, mutexes, and channels.

The organizer itself is not built yet. The current `main.rs` runs one practice module at a time.

## Requirements

- Rust toolchain with edition 2024 support (Rust 1.85 or newer)
- No external crates. `Cargo.toml` has an empty `[dependencies]` section.

## Running

```bash
cargo run
```

Right now this runs `mympsc::test_channels()`. To try another module, comment out the active line in `src/main.rs` and uncomment the one you want.

## Project Structure

```
CLI_File_Organizer/
├── Cargo.toml
├── data/                 # sample text files used by the file system practice
│   ├── file1.txt
│   ├── file2.txt
│   └── file3.txt
└── src/
    ├── main.rs           # entry point; selects which practice module runs
    ├── myfs.rs           # file system practice
    ├── closures.rs       # closure practice
    ├── mymutex.rs        # Mutex and scoped threads practice
    └── mympsc.rs         # mpsc channel practice
```

## Modules

### `myfs.rs`: File System

- `test_create_dir()` creates the `./data` directory if it does not exist.
- `create_files()` writes three sample files into `./data`.
- `read_somefile()` reads those files and prints their contents, or the error if one occurs.

### `closures.rs`: Closures

- `test_closures()` defines an add closure, calls it, and prints the result.

### `mymutex.rs`: Mutex

- `test_mutex()` shares a `Mutex<u16>` between two scoped threads. Thread 1 holds the lock and adds to the score with a delay between steps. Thread 2 polls with `try_lock()` until it gets the lock. The final score is printed at the end.

### `mympsc.rs`: Channels

- `test_channels()` sends the numbers 1 to 6 through an `mpsc` channel. A receiver thread reads them with `recv_timeout`. Each timeout adds to a counter that is never reset, so the thread exits after 11 timeouts in total.

## Notes

- The file system functions use relative paths (`./data/...`). Run `cargo run` from the project root, or the file paths will not resolve.
- `target/` is ignored by git (see `.gitignore`).
