####################
## BUILDER        ##
####################

FROM clux/muslrust:stable-2025-04-28 AS pages-builder

# Add our source code.

RUN mkdir -p /workspace/backend/pages
COPY ./shared /workspace/shared
COPY ./backend/ji_core /workspace/backend/ji_core
COPY ./backend/pages /workspace/backend/pages

# Build our application.
WORKDIR /workspace/backend/pages
RUN cargo build --release --no-default-features

####################
## RELEASE        ##
####################

FROM alpine:latest AS release

# Used at runtime
ENV PROJECT_ID=ji-cloud

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=pages-builder \
    /workspace/backend/pages/target/x86_64-unknown-linux-musl/release/ji-cloud-pages \
    /usr/local/bin/cloud-run-app/ji-cloud-pages

COPY ./backend/pages/public /usr/local/bin/cloud-run-app/public
COPY ./backend/pages/templates /usr/local/bin/cloud-run-app/templates


WORKDIR /usr/local/bin/cloud-run-app/

CMD ["./ji-cloud-pages", "release"]

####################
## Sandbox        ##
####################

FROM alpine:latest AS sandbox

# Used at runtime
ENV PROJECT_ID=ji-cloud-developer-sandbox

RUN apk --no-cache add ca-certificates

RUN mkdir /usr/local/bin/cloud-run-app

COPY --from=pages-builder \
    /workspace/backend/pages/target/x86_64-unknown-linux-musl/release/ji-cloud-pages \
    /usr/local/bin/cloud-run-app/ji-cloud-pages

COPY ./backend/pages/public /usr/local/bin/cloud-run-app/public

WORKDIR /usr/local/bin/cloud-run-app/

CMD ["./ji-cloud-pages", "sandbox"]
