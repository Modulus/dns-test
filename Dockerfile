FROM rust:1.54-buster as builder

ENV APP_FOLDER=/opt/app
RUN mkdir "${APP_FOLDER}"

# In this case one does not need to set argument
# The env var RUST_LOG will be picked up by the env_logger in the applicatin and set correctly
ENV RUST_LOG="info"
WORKDIR "${APP_FOLDER}"
COPY Cargo.toml Cargo.lock "${APP_FOLDER}/"
COPY src/ "${APP_FOLDER}/src"
RUN cargo build --release



FROM ubuntu:20.04 as runner

# FROM alpine:3.14

# RUN apk add openssl-dev

RUN apt update && apt install -y libssl-dev ca-certificates


ENV APP_FOLDER=/opt/app
WORKDIR /opt/app


RUN chown -R nobody:nogroup "${APP_FOLDER}"

USER nobody

# USER app
# COPY --chown=app --from="builder" "${APP_FOLDER}/target/release/shotlog" .
COPY --from=builder /opt/app/target/release /opt/app

CMD [ "/opt/app/dns-test"]