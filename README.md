# Rust web API with Warp and Diesel

Simple web API written in Rust with [Warp](https://github.com/seanmonstar/warp) as a web framework and [Diesel](https://github.com/diesel-rs/diesel) for database connection. 

This is the repository for [the blog post](https://sgibala.com/01-01-rust-api-with-warp-and-diesel/).


## Run

### Database

The application needs a running Postgres database. Use local Postgres or run it in a Docker container:
```bash
docker run -p 5432:5432 --rm -e POSTGRES_PASSWORD=password postgres:12
```

When the database is running make sure its connection string matches the one in `.env` file.

Use Diesel CLI to setup database and run migrations:
```bash
diesel database setup
```

### Application

Before starting the application set `DATABASE_URL` environment variable. If the database connection string matches the one in `.env` file, run:
```bash
export $(cat .env | xargs)
```

Run application with `cargo`:
```bash
cargo run
```
