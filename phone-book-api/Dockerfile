FROM rust:1.67 as build

RUN USER=root cargo new --bin phone-book-user
WORKDIR /usr/src/app

ENV CARGO_HOME=/usr/local/cargo

COPY Cargo.toml .
COPY Cargo.lock .
COPY ./src ./src

RUN --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/registry/index \
    cargo build --release


# COPY --from=build target/release/phone-book-api .
EXPOSE 3000
CMD [ "cargo", "run"]
# CMD ["cargo", "run"]










