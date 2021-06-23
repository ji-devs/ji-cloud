## How to run this API locally
CI - 002

* copy `.env.sample` to `.env`

* fill in all the variables there

spin up a MinIO server
```bash
docker run -p 9000:9000 \
  --name minio1 \
  -v /mnt/data:/data \
  -e "MINIO_ACCESS_KEY=this-is-a-key" \
  -e "MINIO_SECRET_KEY=password1" \
  minio/minio server /data
```
(note, this is a temp server,
 when the container is closed the data in it is lost, if this is not what you want,
 see https://docs.min.io/docs/minio-docker-quickstart-guide.html
)

create s3 bucket
```
cargo run --bin kibibyte
```

```bash
cargo run
```