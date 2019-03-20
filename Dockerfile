FROM rust:1.33.0-slim-stretch AS stage-build
RUN mkdir -p /app/build \
	&& apt-get update \
	&& apt-get install -y build-essential musl-tools \
	&& rustup default nightly-2019-03-15 \
	&& rustup target add x86_64-unknown-linux-musl
WORKDIR /app/build
ADD . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.7
RUN mkdir -p /app/bin
COPY --from=stage-build /app/build/target/x86_64-unknown-linux-musl/release/oxidated-soha-blog /app/bin/
CMD ["/app/bin/oxidated-soha-blog"]
