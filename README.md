cargo-path
==========

`cargo-path` is a command-line utility that prints the path to a local Rust
dependency.

It is invoked as `cargo path <DEPENDENCY>`. For example:

```console
$ cargo path clap
/Users/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/clap-4.6.1
```

When combined with the appropriate exhortation, it is extremely helpful for
Rust development with coding agents.

With `cargo-path`, agents will immediately consult local sources for
dependencies, which are guaranteed to be up-to-date for the correct version.

Without `cargo-path`, agents tend to search the web for dependency
documentation, which is slow and often turns up out-of-date information.

AGENTS.md Snippet
-----------------

Always consult the local source code for information about Rust dependencies,
which is guaranteed to be up-to-date for the correct version.

Run `cargo path NAME` to find the source directory for a dependency:

```console
$ cargo path clap
/Users/home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/clap-4.6.1
```
