FROM rust:slim-stretch AS stage-build

RUN mkdir -p /app/build \
	&& apt-get update \
	&& apt-get install -y build-essential libpq-dev libpq5 \
	&& rustup default nightly
WORKDIR /app/build
ADD . .
RUN cargo build --release

# main
FROM debian:stretch-slim
RUN mkdir -p /app/bin \
	&& apt-get update \
	&& apt-get install -y libpq5 \
	&& apt-get autoremove -y && apt-get clean && apt-get autoclean && rm -rf /tmp/* /var/tmp/* /var/lib/apt/lists/*  /var/cache/apt/archives/*.deb /var/cache/apt/archives/partial/*.deb /var/cache/apt/*.bin
COPY --from=stage-build /app/build/target/release/sohablog /app/bin/
CMD ["/app/bin/sohablog"]
