# DirMap

DirMap is a command-line tool written in Rust that maps out the content of directories in a hierarchical structure. It allows you to specify the starting directory and the maximum depth of exploration. The result is then written to a specified output file.

## Installation

Before you can use DirMap, you need to have Rust and its package manager Cargo installed. You can download Rust from the [official website](https://www.rust-lang.org/).

Clone this repository to your local machine:

```
git clone https://github.com/yourusername/dirmap.git
```

Navigate to the project's root directory:

```
cd dirmap
```

Then, you can build and install DirMap using Cargo:

```
cargo build --release
cargo install --path .
```


The binary will be located in the `target/release/` directory. The `cargo install` command will copy the binary to a directory in your `PATH`, so you can run it from anywhere.

## Usage

Run DirMap with the `-p`, `-o`, and `-d` options to specify the starting path, output file, and max depth, respectively:

```
dirmap -p <starting_path>(=".") -o <output_file>(="output.txt") -d <max_depth>(=10)
```


For example:

```
dirmap -p /home/user/documents -o output.txt -d 3
```


This will explore the `/home/user/documents` directory down to a maximum depth of 3 levels, and write the resulting map to `output.txt`.

## Cross-compiling

If you want to compile DirMap for a different operating system or architecture, you'll need to add a target for that system and build for that target. This can get complex, especially if you're targeting a different OS, because you may need to set up a toolchain for that OS. Tools like [cross](https://github.com/rust-embedded/cross) can make this process easier. Once you've cross-compiled DirMap, you can distribute the resulting binary to users on the target system.

## Contributing

This is a simple project for learning purposes, contributing should not be necessary, if interested in something more elaborate I highly suggest [walkdir](https://docs.rs/walkdir/latest/walkdir/).
