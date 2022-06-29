#!/bin/bash
set -e

ulimit -n 65000
cargo sqlx migrate run
cargo test
