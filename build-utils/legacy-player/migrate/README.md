# Process

In each of these there are various options for output files, testing config, etc.

Generally run `cargo +nightly run --bin [binary name]`

## 1. On GCE or some machine
1a. download_albums: this downloads the _metadata_ for each album from tiny tap. In other words, gives us the list of available jigs

1b. parse: the heavy lifting: for each album, it parses TT data into Jigzi data, writes the output into local files, downloads media, and transcodes.
    - pay special attention to the errors output here. May want to delete those folders from games since they are corrupt. Though in many cases an error here means it couldn't create the folder to begin with.
    - there are several quirks at this stage: https://github.com/ji-devs/ji-cloud/discussions/1787

## 2. From dev or anywhere (loads remote)
2a. create_jig: actually creates the jigs on jigzi, using the output from `parse`
    - pay special attention to the info.txt file output here. It can be copied into a `skip list` for re-running without causing duplicates as well as for the next step.
    - also specify the errors log from parse step

2b (optional). update jig: isn't being used as of right now, and would probably change over time. The idea is you give it the `info.txt` from `create_jig` and it will load the original game.json as well as have info for the jig id and hash, so we can update the jigs without having to run any step besides this.

# Some helpful commands
cargo +nightly watch -w ../../../frontend/apps/crates/utils -w ../../../shared/rust -w ./src -w ./tests -x "test -- --nocapture"

cargo +nightly watch -w ../../../frontend/apps/crates/utils -w ../../../shared/rust -w ./src -w ./tests -x "run -- --bin [binary name]"
