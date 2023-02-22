FROM rust:slim-buster AS builder

WORKDIR /prod
COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir .cargo

COPY . .
RUN cargo build --release

FROM fedora:34 as runner
COPY --from=builder /prod/target/release/means-to-an-end /bin
EXPOSE 8080
CMD /bin/means-to-an-end
