##########
## TEST ##
##########
# Even though we're not building a static binary
# Use the same environment in tests and release
FROM ekidd/rust-musl-builder:latest AS tests 

# Add our source code.

RUN mkdir -p ./backend/api
COPY ./shared ./shared
COPY ./backend/api ./backend/api

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
WORKDIR ./backend/api/main
RUN cargo test

####################
## RELEASE        ##
####################

FROM ekidd/rust-musl-builder:latest AS release-builder

# Add our source code.

RUN mkdir -p ./backend/api
COPY ./shared ./shared
COPY ./backend/api ./backend/api

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
WORKDIR ./backend/api/main
RUN cargo build --release --features release

# Now, we need to build our _real_ Docker container, copying in the binary.
FROM alpine:latest as release

# Used at runtime
ENV PROJECT_ID=ji-cloud

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=release-builder \
    /home/rust/src/backend/api/target/x86_64-unknown-linux-musl/release/ji-cloud-api \
    /usr/local/bin/cloud-run-app/ji-cloud-api

WORKDIR /usr/local/bin/cloud-run-app/


CMD ["./ji-cloud-api"]

####################
## Sandbox        ##
####################

FROM ekidd/rust-musl-builder:latest AS sandbox-builder

# Add our source code.

RUN mkdir -p ./backend/api
COPY ./shared ./shared
COPY ./backend/api ./backend/api

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
WORKDIR ./backend/api/main
RUN cargo build --release --features sandbox

# Now, we need to build our _real_ Docker container, copying in the binary.
FROM alpine:latest as sandbox 

# Used at runtime
ENV PROJECT_ID=ji-cloud-developer-sandbox

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=sandbox-builder \
    /home/rust/src/backend/api/target/x86_64-unknown-linux-musl/release/ji-cloud-api \
    /usr/local/bin/cloud-run-app/ji-cloud-api

WORKDIR /usr/local/bin/cloud-run-app/


CMD ["./ji-cloud-api"]