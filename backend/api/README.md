## How to run this API locally

* copy `.env.sample` to `.env`

* fill in all the variables there

spin up a MinIO server (optional)
```bash
docker run -p 9000:9000 \
  --name minio1 \
  -v /mnt/data:/data \
  -e "MINIO_ACCESS_KEY=this-is-a-key" \
  -e "MINIO_SECRET_KEY=password1" \
  minio/minio server /data
```
run postgres database
```bash
docker run -d \
                                                  --name postgres \
                                                  -p 5432:5432 \
                                                  -e POSTGRES_PASSWORD=password \
                                                  -e POSTGRES_DB=ji-jicloud-dev \
                                                  --restart=always \
                                                  postgres:12

```
note: no redis necessary for this API

run sqlx database (from api folder, run this command)
```bash
sqlx migrate run
```

##Reset database
reset the database after creating/editing a database file
```bash
sqlx db drop -y && sqlx db create && sqlx migrate run
```

Add final edits and save query data to sqlx-data.json in the current director
```bash
cargo sqlx prepare -- --lib
cargo sqlx prepare -- --test integration
```



##How to test API after getting it to run locally
if you haven't done so already, install cargo-insta. This will be used to keep track of database edits and changes in the tests.

```bash
cargo install cargo-insta
```

raise file descriptor limit on linux (optional - if OS error 24)
```bash
ulimit -nH 65000
ulimit -nS 65000
```
Optional: Run postgres in memory
```bash
docker run --name postgres12 --tmpfs /var/lib/postgresql/data -p 5432:5432 -e POSTGRES_PASSWORD=password -d postgres:12 -c max_connections=250
```

To run tests:
```bash
export DATABASE_URL=postgres://postgres:password@localhost/
sqlx migrate run
cargo test
```
