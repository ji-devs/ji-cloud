# Database setup

There are 3 different database targets: local, release, and sandbox.

Local means totally local to the developer machine, local password, etc. 
Release and sandbox are on google cloud and require access through google-cloud-proxy

The connection string needs to be set in different places depending on the project scope, which is covered in the rest of this Setup chapter

Note that "local" is up to the dev... could be in Docker, or native, it doesn't matter

# Cloud Sql Proxy

Although the username, password, and database name are set in [config/.env](../config/config.md) files, the database instance name needs to be passed as a commandline arg to cloud-sql-proxy.

Set this in `build-utils/package.json`. Note that the port and instance should match `SQL_PROXY_PORT` and `DB_INSTANCE_*` in the [config](../config/config.md) too. 

# Local Docker

Although local _can_ be anything (including native), a docker setup is provided in `build-utils/db_local`. Simply `cargo make db-local`

Make sure to not conflict with other ports of other docker instances, of course :)

