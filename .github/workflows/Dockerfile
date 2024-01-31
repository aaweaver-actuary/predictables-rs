FROM rust:1.75.0 as builder

WORKDIR /home/runner/work/predictables-rs/predictables-rs
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /home/runner/work/predictables-rs/predictables-rs/cargo/bin/myapp /home/runner/work/predictables-rs/predictables-rs/bin/
CMD ["myapp"]

