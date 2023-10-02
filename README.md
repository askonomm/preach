# Preach

A minimal, opinionated blogging tool written in Rust.

## Install

Since Preach is currently very early in development (although I use it for my own site already at [asko.dev](https://asko.dev)) I don't yet bother providing distributable binaries myself, so to get them you must install Rust and Cargo yourself, then clone this repo and run `cargo build --release` and look into the `target/release` directory for the binary.

Oh and, you must have a `DATABASE_URL` environment variable set, pointing to a PostgreSQL database. Once you run Preach it will automatically run migrations and set things up. All you have to do once Preach is running, is to go to `/admin/setup` path and create the initial admin account.
