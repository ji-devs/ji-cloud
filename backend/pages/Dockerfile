##########
## TEST ##
##########
# Even though we're not building a static binary
# Use the same environment in tests and release
FROM ekidd/rust-musl-builder:latest AS tests 

# Add our source code.

RUN mkdir -p ./backend/pages
COPY ./shared ./shared
COPY ./config/rust ./config/rust
COPY ./backend/_core/rust ./backend/_core/rust
COPY ./backend/pages ./backend/pages

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
WORKDIR ./backend/pages
RUN cargo test

####################
## RELEASE        ##
####################

FROM ekidd/rust-musl-builder:latest AS release-builder

# Add our source code.

RUN mkdir -p ./backend/pages
COPY ./shared ./shared
COPY ./config/rust ./config/rust
COPY ./backend/_core/rust ./backend/_core/rust
COPY ./backend/pages ./backend/pages


# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
WORKDIR ./backend/pages
RUN cargo build --release --no-default-features

# Now, we need to build our _real_ Docker container, copying in the binary.
FROM alpine:latest as release

# Used at runtime
ENV PROJECT_ID=ji-cloud

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=release-builder \
    /home/rust/src/backend/pages/target/x86_64-unknown-linux-musl/release/ji-cloud-pages \
    /usr/local/bin/cloud-run-app/ji-cloud-pages

COPY ./backend/pages/public /usr/local/bin/cloud-run-app/public
COPY ./backend/pages/templates /usr/local/bin/cloud-run-app/templates


WORKDIR /usr/local/bin/cloud-run-app/


CMD ["./ji-cloud-pages", "release"]

####################
## Sandbox        ##
####################

FROM ekidd/rust-musl-builder:latest AS sandbox-builder

# Add our source code.

RUN mkdir -p ./backend/pages
COPY ./shared ./shared
COPY ./config/rust ./config/rust
COPY ./backend/_core/rust ./backend/_core/rust
COPY ./backend/pages ./backend/pages


# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
WORKDIR ./backend/pages
RUN cargo build --release --no-default-features

# Now, we need to build our _real_ Docker container, copying in the binary.
FROM alpine:latest as sandbox 

# Used at runtime
ENV PROJECT_ID=ji-cloud-developer-sandbox

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=sandbox-builder \
    /home/rust/src/backend/pages/target/x86_64-unknown-linux-musl/release/ji-cloud-pages \
    /usr/local/bin/cloud-run-app/ji-cloud-pages

COPY ./backend/pages/public /usr/local/bin/cloud-run-app/public

WORKDIR /usr/local/bin/cloud-run-app/

CMD ["./ji-cloud-pages", "sandbox"]
