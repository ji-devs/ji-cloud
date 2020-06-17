# Database setup

There are 3 different database targets: local, release, and sandbox.

Local means totally local to the developer machine, local password, etc. 
Release and sandbox are on google cloud and require access through google-cloud-proxy

The connection string needs to be set in different places depending on the project scope, which is covered in the rest of this Setup chapter

# Database Migration

While the diesel migrations are setup in the Rust project that needs the schemas, actually running the migrations on the target database is done via a standalone tool: `build-utils/db_migrations`

From that directory, run `cargo run --target [local/release/sandbox]`

it will run the migrations from the `backend/api` diesel project against the target database

if it's really necessary to connect directly to the google cloud sql, the connection string with the password can be output via passing `--connection_string_only` (this requires that the local credentials exist, of course)
