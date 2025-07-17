####################
## BUILDER        ##
####################

FROM  blackdex/rust-musl:x86_64-musl AS api-builder

# Use crates.io sparse protocol
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Add our source code.
RUN mkdir -p /workspace/backend/api
COPY ./shared /workspace/shared
COPY ./backend/ji_core /workspace/backend/ji_core
COPY ./backend/api /workspace/backend/api

# Build our application.
WORKDIR /workspace/backend/api
RUN cargo build --release --no-default-features

####################
## Release        ##
####################

FROM alpine:latest AS release

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

FROM alpine:latest AS sandbox

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

FROM alpine:latest AS release_media_watch

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

FROM alpine:latest AS sandbox_media_watch

# Used at runtime
ENV PROJECT_ID=ji-cloud-developer-sandbox

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=api-builder \
    /workspace/backend/api/target/x86_64-unknown-linux-musl/release/media-watch \
    /usr/local/bin/cloud-run-app/media-watch

WORKDIR /usr/local/bin/cloud-run-app/


CMD ["./media-watch", "sandbox"]
