ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# builder stage
FROM ${BASE_IMAGE} AS builder

ADD --chown=rust:rust Cargo.lock ./
ADD --chown=rust:rust Cargo.toml ./
ADD --chown=rust:rust doc/doc.html ./doc/
ADD --chown=rust:rust src/ ./src/

RUN cargo build --release

# final stage
FROM alpine:latest

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/api-epi-today \
    /usr/local/bin/

EXPOSE 4242

CMD ["/usr/local/bin/api-epi-today"]