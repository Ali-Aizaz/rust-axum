# Rust Axum BackEnd Template 

The code is available under either the MIT or Apache license and is free to use.

Important: Make sure to refresh this repo. I implemented a fix on `cookie.add` for `AUTH_TOKEN` (see details below in the notes section).

## Dev (REPL)

```sh

# Terminal - To run the server.
cargo watch -q -c -w src/ -x "run"

```

## Dev

```sh

# Terminal - To run the server.
cargo run

```

## Notes

- IMPORTANT - for `AUTH_TOKEN` cookie, make sure to call `cookie.set_path("/");`.
- Use the `--poll` flag for cargo watch (latest 8.4.0) on my Fedora Linux environment.
- Make sure to use `axum` version `0.6.16` or above, as version `0.6.15` had a bug in the static routing.

This repository can be found on [GitHub](https://github.com/Ali-Aizaz/rust-axum).
