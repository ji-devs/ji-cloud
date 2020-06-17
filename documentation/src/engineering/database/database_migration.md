# Database Migration

use `build-utils/db_migrations`

`cargo run --target [local/release/sandbox]`

it will run the migrations from the `backend/api` diesel project against the target database

if it's really necessary to connect directly to the google cloud sql, the connection string with the password can be output via passing `--connection_string_only` (this requires that the local credentials exist, of course)

