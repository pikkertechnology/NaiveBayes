ARG RUST_VERSION=1.83.0
ARG APP_NAME=NaiveBayes

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache clang lld musl-dev git

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/naivebayes

FROM alpine:3.18 AS final

COPY cleanup.sh /usr/local/bin/cleanup.sh
RUN chmod +x /usr/local/bin/cleanup.sh

RUN echo "0 0 * * * /usr/local/bin/cleanup.sh" >> /var/spool/cron/crontabs/root

COPY --from=build /bin/naivebayes /bin/

EXPOSE 8080

CMD crond -b && /bin/naivebayes