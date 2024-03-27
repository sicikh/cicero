# Cicero Project

Web-app for using Cicero DSL to generate documents interactively.

# Prerequisites

- install [Rust](https://www.rust-lang.org/tools/install) nightly toolchain with WASM support:

```sh
$ rustup toolchain install nightly
$ rustup default nightly
$ rustup target add wasm32-unknown-unknown
```

- install [cargo-leptos](https://github.com/leptos-rs/cargo-leptos):

```sh
$ cargo install cargo-leptos
```

- install Tailwind CSS:

```sh
$ npm -i tailwindcss -g
```

- install [Tectonic](https://tectonic-typesetting.github.io/en-US/) for rendering into PDF (tested on version 0.15.0)

- install [pandoc](https://pandoc.org/installing.html) for rendering into DOCX

# Running in development

Run application in `watch` mode via `cargo-leptos`:

```sh
$ cargo leptos watch
```

# Building for production

Build application in `release` mode via `cargo-leptos`:

```sh
$ cargo leptos build --release
```
