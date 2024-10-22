# Electrical consumption analyzer API

When the program is running, the simulated data is traced.
The data we successfully parse is logged as info, and the corrupted data is logged as an error.

The '/top-consumers' page shows the top consumer data.
It does not support live rendering, so you need to refresh it to get the updated data.
The consumers it shows are not from a recent time period as specified in the description, but from the time the server has been running.

The other functionality is not implemented.

## Prerequisites

[Install Rust and Cargo via Rustup](https://www.rust-lang.org/tools/install).

## Running

To run:

```sh
cargo run
```

## Development

The server runs on `localhost:3000`.
