FROM rust:1.67

WORKDIR /usr/src/smoke-test
COPY . .

RUN cargo install --path .

CMD ["smoke-test"]
