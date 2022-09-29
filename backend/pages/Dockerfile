ARG ZIG_VERSION="0.9.1"
ARG ALPINE_VERSION="3.16.0"
ARG MOLD_VERSION="1.3.1"

####################
## BASE           ##
####################
#
# This stage does a bit of extra work to try and improve layer caching and build performance. See individual comments for
# more details.
#
# The base image also has cargo-zigbuild and ziglang installed for _much_ easier cross-compilation to musl.
#
# rust nightly is used for access to the sparse-registry unstable flag.
FROM rustlang/rust:nightly-buster AS base
# Enabling the `-Z sparse-registry` flag considerably reduces the amount of time it takes for the crates-io registry to be
# updated. For more information, have a look at https://internals.rust-lang.org/t/call-for-testing-cargo-sparse-registry/16862
ENV CARGO_UNSTABLE_SPARSE_REGISTRY=true
# Make these arguments available to this stage
ARG MOLD_VERSION
ARG ZIG_VERSION
# mold drastically improves linking times.
# Use mold whenever we're not using zigbuild.
RUN wget \
    -q \
    -O- \
    "https://github.com/rui314/mold/releases/download/v$MOLD_VERSION/mold-$MOLD_VERSION-$(uname -m)-linux.tar.gz" \
    | tar \
        -C /usr/local \
        --strip-components=1 \
        -xzf \
        -
RUN ln -sf /usr/local/bin/mold $(realpath /usr/bin/ld)

# We need the musl toolchain available for cross-compiling with zig
RUN rustup target add x86_64-unknown-linux-musl

# Install Zig
RUN cargo +nightly install cargo-zigbuild
RUN curl -L "https://ziglang.org/download/${ZIG_VERSION}/zig-linux-$(uname -m)-${ZIG_VERSION}.tar.xz" | tar -J -x -C /usr/local
RUN ln -s "/usr/local/zig-linux-$(uname -m)-${ZIG_VERSION}/zig" /usr/local/bin/zig

# Create the project dirs so that we don't need to later
RUN mkdir -p /shared/rust/src
RUN mkdir -p /backend/core/src
RUN mkdir -p /backend/pages/src

####################
## DEPENDENCIES   ##
####################
#
# Build just the dependencies.
#
# We create dummy crates with just the Cargo.toml files and a lib.rs, and tell all the dependencies for these crates. In
# the actual build step, we then update the timestamps on the Cargo.toml and lib.rs files so that cargo can rebuild the
# _local_ dependencies and the app itself.
# This has a drawback that a change in backend/pages will trigger a rebuild for the shared/rust crate, but
# overall, that's an easy price to pay.
FROM base AS dependencies
COPY ./shared/rust/Cargo.toml /shared/rust/Cargo.toml
RUN touch /shared/rust/src/lib.rs

COPY ./backend/core/Cargo.toml /backend/core/Cargo.toml
RUN touch /backend/core/src/lib.rs

COPY ./backend/pages/Cargo.toml /backend/pages/Cargo.toml
# For the main pages package, we need to include the lock file so that cargo builds the correct versions of our dependencies.
COPY ./backend/pages/Cargo.lock /backend/pages/Cargo.lock
RUN touch /backend/pages/src/lib.rs

# Build just our dependencies
WORKDIR /backend/pages
RUN cargo +nightly zigbuild --release --target x86_64-unknown-linux-musl

####################
## BUILDER        ##
####################
#
# Build the actual project.
#
# This stage copies _all_ our project files over, and also copies the entire target directory generated in the dependencies
# stage. `cargo build` does the rest of the work.
FROM base AS builder
COPY ./shared /shared
COPY ./backend/core /backend/core
COPY ./backend/pages /backend/pages
COPY --from=dependencies /backend/pages/target /backend/pages/target

# We need to touch these files so that their timestamps are newer than what was used in the dependencies layer, otherwise
# cargo will not know to rebuild these crates. See the comment in the dependencies stage.
# Very hacky indeed.
RUN touch /shared/rust/Cargo.toml
RUN touch /shared/rust/src/lib.rs
RUN touch /backend/core/Cargo.toml
RUN touch /backend/core/src/lib.rs
RUN touch /backend/pages/Cargo.toml
RUN touch /backend/pages/Cargo.lock
RUN touch /backend/pages/src/lib.rs

# Build the actual application
WORKDIR /backend/pages
RUN cargo +nightly zigbuild --release --no-default-features --target x86_64-unknown-linux-musl

####################
## App            ##
####################
#
# Creates a minimal image for running the app, but without setting the entrypoint or command to run.
# When running the container, the `PROJECT_ID` should be passed in as an environment variable, and the
# command should be set.
#
# Valid project ID's are:
# - ji-cloud (release)
# - ji-cloud-developer-sandbox (sandbox)
#
# Valid commands:
# - ji-cloud-pages [release|sandbox]
FROM alpine:$ALPINE_VERSION as app

RUN apk --no-cache add ca-certificates

RUN mkdir -p /usr/local/bin

COPY --from=builder \
    /backend/pages/target/x86_64-unknown-linux-musl/release/ji-cloud-pages \
    /usr/local/bin/ji-cloud-pages

COPY ./backend/pages/public /usr/local/bin/cloud-run-app/public
COPY ./backend/pages/templates /usr/local/bin/cloud-run-app/templates

WORKDIR /usr/local/bin/
