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
sqlx db migrate run
```

##Reset database
```bash
sqlx db drop -y && sqlx db create && sqlx migrate run
```

## How to test API after getting it to run locally
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
sqlx db migrate run
cargo test
```
