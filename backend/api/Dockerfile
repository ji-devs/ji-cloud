####################
## BUILDER        ##
####################

FROM ekidd/rust-musl-builder:latest AS api-builder

# Add our source code.

RUN mkdir -p ./backend/api
COPY ./shared ./shared
COPY ./backend/core ./backend/core
COPY ./backend/api ./backend/api

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
WORKDIR ./backend/api
RUN cargo build --release --no-default-features

####################
## Release        ##
####################

FROM alpine:latest as release

# Used at runtime
ENV PROJECT_ID=ji-cloud

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=api-builder \
    /home/rust/src/backend/api/target/x86_64-unknown-linux-musl/release/ji-cloud-api \
    /usr/local/bin/cloud-run-app/ji-cloud-api

WORKDIR /usr/local/bin/cloud-run-app/


CMD ["./ji-cloud-api", "release"]

####################
## Sandbox        ##
####################

FROM alpine:latest as sandbox

# Used at runtime
ENV PROJECT_ID=ji-cloud-developer-sandbox

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=api-builder \
    /home/rust/src/backend/api/target/x86_64-unknown-linux-musl/release/ji-cloud-api \
    /usr/local/bin/cloud-run-app/ji-cloud-api

WORKDIR /usr/local/bin/cloud-run-app/


CMD ["./ji-cloud-api", "sandbox"]



#####################
## Release (media) ##
#####################

FROM alpine:latest as release_media_watch

# Used at runtime
ENV PROJECT_ID=ji-cloud

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=api-builder \
    /home/rust/src/backend/api/target/x86_64-unknown-linux-musl/release/media-watch \
    /usr/local/bin/cloud-run-app/media-watch

WORKDIR /usr/local/bin/cloud-run-app/


CMD ["./media-watch", "release"]

#####################
## Sandbox (media) ##
#####################

FROM alpine:latest as sandbox_media_watch

# Used at runtime
ENV PROJECT_ID=ji-cloud-developer-sandbox

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=api-builder \
    /home/rust/src/backend/api/target/x86_64-unknown-linux-musl/release/media-watch \
    /usr/local/bin/cloud-run-app/media-watch

WORKDIR /usr/local/bin/cloud-run-app/


CMD ["./media-watch", "sandbox"]
