# Y Server
Y is a simplified, open source clone of X. It is written in pure Rust and is yet another project I use to level up my Rust skills. This repo stores the code of the Y server, which is separate from the client app. The client has not started being developed yet. When it does, I will include the link here.

# Building and running
**Before building, make sure to have the Rust toolchain installed, namely cargo and the Rust compiler.**
Database setup scripts are not yet available, so you need to setup PostgreSQL manually. Then, run
```sh
cargo install sqlx-cli
cargo sqlx migrate run
```
To generate the required tables for the database.
And then, as with any other Rust project:
```sh
cargo run
```
It is worth mentioning that you need to specify a .env file containing DATABASE_URL and Y_SERVER_PORT variables. Or you could use the provided one, just make sure that credentials match.

# Roadmap
- [ ] A basic feed
- [ ] User pages
- [ ] Images in posts
- [ ] Post creation, editing
- [ ] Timed login session
- [x] User registration
