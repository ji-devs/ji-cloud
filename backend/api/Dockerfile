####################
## BUILDER        ##
####################

FROM ekidd/rust-musl-builder:latest AS api-builder

# Add our source code.

RUN mkdir -p ./backend/api
COPY ./shared ./shared
COPY ./config/rust ./config/rust
COPY ./backend/core ./backend/core
COPY ./backend/api ./backend/api

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

RUN rm -r backend/api/integration-testing backend/api/fixtures

# Build our application.
WORKDIR ./backend/api
RUN cargo build --release --no-default-features

##########
## TEST ##
##########

FROM alpine:latest as test

RUN apk --no-cache add yarn bash postgresql postgresql-contrib

COPY ./backend/script/ephemeralpg/pg_tmp.sh \ 
    ./backend/api/integration-testing \
    integration-testing/

COPY ./backend/api/fixtures/ integration-testing/fixtures

COPY --from=api-builder \
    /home/rust/src/backend/api/target/x86_64-unknown-linux-musl/release/ji-cloud-api \
    ./integration-testing/ji-cloud-api

WORKDIR ./integration-testing
RUN yarn && chown -R postgres:postgres .
RUN su postgres -c 'BIN_FILE="./ji-cloud-api" PG_TMP="./pg_tmp.sh" FIXTURES_DIR=fixtures yarn test:ava --timeout 2m'

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
