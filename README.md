<p align="center">
<img src="https://github.com/Javatrix/y-server/blob/main/y.webp" width="25%" alt="tsuki-chan"/>
</p>

<h1 align="center">Y Server</h1>

Y is a simplified, open source clone of X. It is written in pure Rust and is yet another project I use to level up my Rust skills. This repo stores the code of the Y server, which is separate from the client app. The client has not started being developed yet. When it does, I will include the link here.

# Building and running
**Prerequisites:**
- Rust toolchain
- PostgreSQL
- Git

```sh
git clone https://github.com/Javatrix/y-server
cd y-server
```
Database setup scripts are not yet available, so you need to set it up manually.
To do so, enter the PostgreSQL prompt and paste in these commands:
```sql
CREATE DATABASE y_db;
```
Quit with \q, and run `psql y_db` to enter the database.
Next, setup the admin account:
```sql
CREATE USER y_admin;
GRANT USAGE ON SCHEMA public TO y_admin;
GRANT CREATE ON SCHEMA public TO y_admin;
ALTER USER y_admin WITH PASSWORD 'just_an_x_clone';
```
This should prepare the database for the automated setup.
Back in the root project directory run
```sh
cargo install sqlx-cli
cargo sqlx migrate run
```
to generate the required tables for the database.
And then, as with any other Rust project:
```sh
cargo run
```

# Roadmap
- [ ] Containerization
- [ ] A basic feed
- [ ] User pages
- [ ] Images in posts
- [ ] Post editing/removing
- [x] Post creation
- [x] Timed login session
- [x] User registration
