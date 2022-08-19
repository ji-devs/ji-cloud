## docker build -t otel-custom:latest \
##   --build-arg arg_honeycomb_write_key=<write_key> \
##   --build-arg arg_honeycomb_dataset=jigzi_local .
##
## docker run -p 4317:4317 --env HONEYCOMB_SAMPLE_RATE="0.2" otel-custom:latest

FROM otel/opentelemetry-collector-contrib:0.45.0

ARG arg_honeycomb_write_key
ARG arg_honeycomb_dataset

ENV HONEYCOMB_WRITE_KEY=$arg_honeycomb_write_key
ENV HONEYCOMB_DATASET=$arg_honeycomb_dataset

COPY ./otel-config.yaml /etc/otel/config.yaml

CMD ["--config", "/etc/otel/config.yaml"]
