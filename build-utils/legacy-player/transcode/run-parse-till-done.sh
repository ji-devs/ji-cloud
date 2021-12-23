#!/bin/sh
until cargo +nightly run --bin parse
do
        echo "crashed with exit code $?.  Respawning.." >&2
        sleep 10
done