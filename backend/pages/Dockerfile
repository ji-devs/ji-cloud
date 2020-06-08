##########
## TEST ##
##########
# Even though we're not building a static binary
# Use the same environment in tests and release
FROM ekidd/rust-musl-builder:latest AS tests 

# Add our source code.

RUN mkdir -p ./backend/pages
COPY ./shared ./shared
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
COPY ./backend/pages ./backend/pages


# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
WORKDIR ./backend/pages
RUN cargo build --release --features release

# Now, we need to build our _real_ Docker container, copying in the binary.
FROM alpine:latest as release

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=release-builder \
    /home/rust/backend/pages/src/target/x86_64-unknown-linux-musl/release/ji-cloud-pages \
    /usr/local/bin/cloud-run-app/ji-cloud-pages

COPY ./backend/pages/public /user/local/bin/cloud-run-app/public
COPY ./backend/pages/handlebars /user/local/bin/cloud-run-app/handlebars 


WORKDIR /usr/local/bin/cloud-run-app/

CMD ["./ji-cloud-pages"]

####################
## Sandbox        ##
####################

FROM ekidd/rust-musl-builder:latest AS sandbox-builder

# Add our source code.

ADD . ./

# Fix permissions on source code.
# RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release --features sandbox

# Now, we need to build our _real_ Docker container, copying in the binary.
FROM alpine:latest as sandbox 

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=sandbox-builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/ji-cloud-pages \
    /usr/local/bin/cloud-run-app/ji-cloud-pages

COPY --from=sandbox-builder \
    /home/rust/src/public \
    /usr/local/bin/cloud-run-app/public

COPY --from=sandbox-builder \
    /home/rust/src/handlebars \
    /usr/local/bin/cloud-run-app/handlebars

WORKDIR /usr/local/bin/cloud-run-app/

CMD ["./ji-cloud-pages"]
