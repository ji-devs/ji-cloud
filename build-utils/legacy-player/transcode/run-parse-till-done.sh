#!/bin/sh
until cargo +nightly run --bin parse
do
        echo "crashed with exit code $?.  Respawning.." >&2
        sleep 1
done