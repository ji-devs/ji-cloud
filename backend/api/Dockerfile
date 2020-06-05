##########
## TEST ##
##########
# Even though we're not building a static binary
# Use the same environment in tests and release
FROM ekidd/rust-musl-builder:latest AS tests 

# Add our source code.

ADD . ./

# Set permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo test

####################
## RELEASE        ##
####################

FROM ekidd/rust-musl-builder:latest AS release-builder

# Add our source code.

ADD . ./

# Set permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release --features release

# Now, we need to build our _real_ Docker container, copying in the binary.
FROM alpine:latest as release
RUN apk --no-cache add ca-certificates
COPY --from=release-builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/ji-cloud-api \
    /usr/local/bin/

# Execute it
CMD ["/usr/local/bin/ji-cloud-api"]


####################
## SANDBOX        ##
####################

FROM ekidd/rust-musl-builder:latest AS sandbox-builder

# Add our source code.

ADD . ./

# Set permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release --features sandbox

# Now, we need to build our _real_ Docker container, copying in the binary.
FROM alpine:latest as sandbox
RUN apk --no-cache add ca-certificates
COPY --from=sandbox-builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/ji-cloud-api \
    /usr/local/bin/

# Execute it
CMD ["/usr/local/bin/ji-cloud-api"]