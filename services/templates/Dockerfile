##########
## TEST ##
##########
# Even though we're not building a static binary
# Use the same environment in tests and release
FROM ekidd/rust-musl-builder:latest AS tests 

# Add our source code.

ADD . ./

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo test

####################
## RELEASE        ##
####################

FROM ekidd/rust-musl-builder:latest AS release-builder

# Add our source code.

ADD . ./

# Fix permissions on source code.
# RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release --features release

# Now, we need to build our _real_ Docker container, copying in the binary.
FROM alpine:latest as release

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=release-builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/ji-cloud-templates \
    /usr/local/bin/cloud-run-app/ji-cloud-templates

COPY --from=release-builder \
    /home/rust/src/public \
    /usr/local/bin/cloud-run-app/public

COPY --from=release-builder \
    /home/rust/src/markup \
    /usr/local/bin/cloud-run-app/markup

WORKDIR /usr/local/bin/cloud-run-app/

CMD ["./ji-cloud-templates"]