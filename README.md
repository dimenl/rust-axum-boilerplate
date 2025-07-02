# Rust Axum Boilerplate

This project is a minimal template using [Axum](https://github.com/tokio-rs/axum) with SeaORM for database access.

## Environment Variables

The application reads configuration from environment variables using `dotenv`. The following variables are required:

- `DATABASE_URL` &ndash; connection string for the database. Example: `postgres://user:password@localhost:5432/app`.
- `ACCESS_TOKEN` &ndash; secret used to sign and verify JWT access tokens.

These variables can be placed in a `.env` file at the project root or exported in your shell.

## Example Commands

Start the development server:

```bash
cargo run
```

Apply database migrations:

```bash
cargo run --bin migrate
```

For automatic reloads during development you can use [`cargo watch`](https://github.com/watchexec/cargo-watch) (see `notes.md`):

```bash
cargo watch -x run
```

