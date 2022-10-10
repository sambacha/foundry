# syntax=docker/dockerfile:1.4
FROM rust:1-alpine3.16 AS build-environment
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"

WORKDIR /opt
RUN set -eux; \
	\
	apk add --no-cache --virtual .foundry-deps \
		ca-certificates \
		musl-dev \
		clang \
        lld \
        curl \
        build-base \
        linux-headers \
        git;

COPY . .
RUN --mount=type=cache,target=/volume/target \
    --mount=type=cache,target=/root/.cargo/registry \
    cargo build --release --locked;
    
RUN strip /opt/foundry/target/release/forge \
    && strip /opt/foundry/target/release/cast \
    && strip /opt/foundry/target/release/anvil;

RUN apk del --no-network .foundry-deps;

FROM alpine:3.16 AS foundry-client
RUN apk upgrade
# if needed, install additional dependencies here
RUN apk add --no-cache libgcc linux-headers gcompat git

COPY --from=build-environment /opt/foundry/target/release/forge /usr/local/bin/forge
COPY --from=build-environment /opt/foundry/target/release/cast /usr/local/bin/cast
COPY --from=build-environment /opt/foundry/target/release/anvil /usr/local/bin/anvil
RUN adduser -Du 1000 foundry

# 8545 is Standard Port
# 8180 is OpenEthereum
# 3001 is a fallback port
EXPOSE 8545/tcp
EXPOSE 8545/udp
EXPOSE 8180
EXPOSE 3001/tcp

STOPSIGNAL SIGQUIT

ENTRYPOINT ["/bin/sh", "-c"]

LABEL org.label-schema.build-date=$BUILD_DATE \
      org.label-schema.name="Foundry" \
      org.label-schema.description="Foundry Toolchain" \
      org.label-schema.url="https://getfoundry.sh" \
      org.label-schema.vcs-ref=$VCS_REF \
      org.label-schema.vcs-url="https://github.com/foundry-rs/foundry.git" \
      org.label-schema.vendor="CommodityStream, Inc" \
      org.label-schema.version=$VERSION \
      org.label-schema.schema-version="1.0"
