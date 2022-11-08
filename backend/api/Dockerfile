####################
## BUILDER        ##
####################

FROM clux/muslrust:stable AS api-builder

# Add our source code.

RUN mkdir -p /workspace/backend/api
COPY ./shared /workspace/shared
COPY ./backend/core /workspace/backend/core
COPY ./backend/api /workspace/backend/api

# Fix permissions on source code.
# RUN sudo chown -R rust:rust /home/rust

# Build our application.
WORKDIR /workspace/backend/api
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
    /workspace/backend/api/target/x86_64-unknown-linux-musl/release/ji-cloud-api \
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
    /workspace/backend/api/target/x86_64-unknown-linux-musl/release/ji-cloud-api \
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
    /workspace/backend/api/target/x86_64-unknown-linux-musl/release/media-watch \
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
    /workspace/backend/api/target/x86_64-unknown-linux-musl/release/media-watch \
    /usr/local/bin/cloud-run-app/media-watch

WORKDIR /usr/local/bin/cloud-run-app/


CMD ["./media-watch", "sandbox"]
