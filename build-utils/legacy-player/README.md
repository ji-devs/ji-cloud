Various ad-hoc processes to download metadata from either tiny tap or jigzi
Edit main.rs in each dir, and comment/run as needed

Consider more like an exploratory tool than anything definitive or stable

And very very loose

There's also some copy and paste and cruft because frankly this was just written as a last-time means-to-an-end 


# Some helpful commands
cargo +nightly watch -w ../../../frontend/apps/crates/utils -w ../../../shared/rust -w ./src -w ./tests -x "test -- --nocapture"

cargo +nightly watch -w ../../../frontend/apps/crates/utils -w ../../../shared/rust -w ./src -w ./tests -x "run -- --bin [binary name]"

### shell script to run forever, restarting on failure
```sh
#!/bin/sh
until cargo +nightly run
do
        echo "crashed with exit code $?.  Respawning.." >&2
        sleep 10
done
```

### gsutil syncing stuff 
_dangerous! with the -d_
gsutil -m rsync -d -r /home/david/archive/legacy-cdn gs://ji-cloud-legacy-eu-001

gsutil -m rsync -r /Users/dakom/Downloads/jigzi-data/games gs://ji-cloud-legacy-eu-001/transcode/games

gsutil -m cp -r /Users/dakom/Downloads/jigzi-data/games/* gs://ji-cloud-legacy-eu-001/transcode/games

gsutil -m cp -r gs://ji-cloud-legacy-eu-001/* /home/david/archive/legacy-cdn

gsutil -m rsync -r /home/david/archive/legacy-cdn gs://ji-cloud-legacy-eu-001