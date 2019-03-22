FROM rust:slim-stretch AS stage-build
RUN mkdir -p /app/build \
	&& apt-get update \
	&& apt-get install -y build-essential \
	&& echo 'deb http://repo.mysql.com/apt/debian/ stretch mysql-8.0' >> /etc/apt/sources.list.d/mysql.list \
	&& echo 'deb http://repo.mysql.com/apt/debian/ stretch mysql-tools' >> /etc/apt/sources.list.d/mysql.list \
	&& apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 8C718D3B5072E1F5 \
	&& apt-get update \
	&& apt-get install -y libmysqlclient-dev \
	&& rustup default nightly
WORKDIR /app/build
ADD . .
RUN cargo build --release

FROM debian:stretch-slim
RUN mkdir -p /app/bin \
	&& apt-get update \
	&& apt-get install -y dirmngr \
	&& echo 'deb http://repo.mysql.com/apt/debian/ stretch mysql-8.0' >> /etc/apt/sources.list.d/mysql.list \
	&& echo 'deb http://repo.mysql.com/apt/debian/ stretch mysql-tools' >> /etc/apt/sources.list.d/mysql.list \
	&& apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 8C718D3B5072E1F5 \
	&& apt-get update \
	&& apt-get install -y libmysqlclient* \
	&& apt-get autoremove -y && apt-get clean && apt-get autoclean && rm -rf /tmp/* /var/tmp/* /var/lib/apt/lists/*  /var/cache/apt/archives/*.deb /var/cache/apt/archives/partial/*.deb /var/cache/apt/*.bin
COPY --from=stage-build /app/build/target/release/soha-blog /app/bin/
ADD templates /app/bin/
CMD ["/app/bin/soha-blog"]
