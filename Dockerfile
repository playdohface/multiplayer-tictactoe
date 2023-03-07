# use cargo-chef to extract the dependencies
FROM rust as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build dependencies separately to be able to cache them
FROM rust as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# build the app
FROM rust AS builder

# Create appuser with low privileges for security
ENV USER=appuser
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

COPY . /app
WORKDIR /app
# use pre-built dependencies to speed up build time
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

#build the app
RUN cargo build --release

# use a lightweight image for running
FROM debian:buster-slim

#copy the user from the builder
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# we need these for some crates to work
RUN apt update && apt install -y libssl-dev libpq-dev ca-certificates

COPY --from=builder /app/target/release/multiplayer-tictactoe /app/multiplayer-tictactoe
# pull in the static files to serve
COPY ./client /app/client

WORKDIR /app

#let's not run as root
USER appuser:appuser

EXPOSE 8080

ENTRYPOINT [ "./multiplayer-tictactoe" ]



