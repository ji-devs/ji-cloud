####################
## BUILDER        ##
####################

FROM ekidd/rust-musl-builder:latest AS pages-builder

# Add our source code.

RUN mkdir -p ./backend/pages
COPY ./shared ./shared
COPY ./backend/core ./backend/core
COPY ./backend/pages ./backend/pages

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
WORKDIR ./backend/pages
RUN cargo build --release --no-default-features

####################
## RELEASE        ##
####################

FROM alpine:latest as release

# Used at runtime
ENV PROJECT_ID=ji-cloud

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=pages-builder \
    /home/rust/src/backend/pages/target/x86_64-unknown-linux-musl/release/ji-cloud-pages \
    /usr/local/bin/cloud-run-app/ji-cloud-pages

COPY ./backend/pages/public /usr/local/bin/cloud-run-app/public
COPY ./backend/pages/templates /usr/local/bin/cloud-run-app/templates


WORKDIR /usr/local/bin/cloud-run-app/

CMD ["./ji-cloud-pages", "release"]

####################
## Sandbox        ##
####################

FROM alpine:latest as sandbox 

# Used at runtime
ENV PROJECT_ID=ji-cloud-developer-sandbox

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=pages-builder \
    /home/rust/src/backend/pages/target/x86_64-unknown-linux-musl/release/ji-cloud-pages \
    /usr/local/bin/cloud-run-app/ji-cloud-pages

COPY ./backend/pages/public /usr/local/bin/cloud-run-app/public

WORKDIR /usr/local/bin/cloud-run-app/

CMD ["./ji-cloud-pages", "sandbox"]
